#![allow(non_snake_case)]

use mjcf::*;
use std::collections::{HashMap, HashSet};

/// Simple test case to play around with.
#[test]
fn proof_of_concept() {
    let test_input = r#"
        <mujoco model="test">
            <size/>
            <compiler autolimits="true" />
            <size/>
            <worldbody>
                <body>
                    <body/>
                </body>
            </worldbody>
        </mujoco>"#;
    dbg!(&test_input);

    let test_value = from_str(test_input).unwrap();
    dbg!(&test_value);

    assert_eq!(test_value.compiler.len(), 1);
    assert_eq!(test_value.size.len(), 2);
    assert_eq!(test_value.asset.len(), 0);

    let test_output = to_string(&test_value).unwrap();
    dbg!(&test_output);

    assert!(test_output.len() <= test_input.len());
    check_xml_histogram(test_input, &test_output);

    // panic!("END OF TEST, CHECK RESULTS MANUALLY!");
}

/// Test the example from the README.
#[test]
fn readme_example() {
    // let mjcf_string = std::fs::read_to_string("my_mujoco_model.xml").unwrap();
    let mjcf_string = r#"<mujoco model="test"><worldbody></worldbody></mujoco>"#;

    let mut mujoco = mjcf::from_str(&mjcf_string).unwrap();
    let worldbody = mujoco.worldbody.as_mut().unwrap();

    let mut new_body = mjcf::Body::default();
    new_body.name = "my_body".to_string();
    worldbody.body.push(new_body);

    let mjcf_string = mjcf::to_string(&mujoco).unwrap();

    // std::fs::write("modified_mujoco_model.xml", mjcf_string).unwrap();
    dbg!(&mjcf_string);

    // panic!("END OF TEST, CHECK RESULTS MANUALLY!");
}

/// Test using MJCF files scraped from the internet.
///
/// This function parses the file into rust, serializes back into XML, and
/// checks that the result looks similar enough.
///
// fn test_mjcf(filename: &str) {
//     // Get and clean up the input file.
//     let filepath = std::path::Path::new(REPO_ROOT).join("mjcf_test_files").join(filename);
//     let input = std::fs::read_to_string(filepath).unwrap();
//     let input = input.replace(">>", ">"); // Apparently MuJoCo will accept malformed XML :(
//     let input = strip_comments(&input);
//     let input = strip_empty_tags(&input);
//     let input = strip_xml_header(&input);
//     dbg!(&input);
//     //
//     let value = from_str(&input).unwrap();
//     dbg!(&value);
//     //
//     let output = to_string(&value).unwrap();
//     dbg!(&output);
//     //
//     check_xml_histogram(&input, &output);
//     // check_joint_order(&input, &output);
// }

const REPO_ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

/// This checks that both files have the same words in them.
/// This does not check the syntax, structure, or order of the words,
/// because attributes in an element can appear in any order, and
/// because most of the elements in the MJCF format are also unordered.
fn check_xml_histogram(input: &str, output: &str) {
    let input_hist = xml_histogram(input);
    let output_hist = xml_histogram(output);

    // Check that both histograms have the same keys.
    let input_words: HashSet<_> = input_hist.keys().collect();
    let output_words: HashSet<_> = output_hist.keys().collect();
    let mut missing_words: Vec<_> = input_words.difference(&output_words).collect();
    let mut extra_words: Vec<_> = output_words.difference(&input_words).collect();
    missing_words.sort_unstable();
    extra_words.sort_unstable();
    assert!(missing_words.is_empty(), "MISSING WORDS ({}): {:#?}", missing_words.len(), missing_words);
    assert!(extra_words.is_empty(), "EXTRA WORDS ({}): {:#?}", extra_words.len(), extra_words);

    // Check that both histogram have the same values.
    let mut all_words: Vec<_> = input_hist.keys().collect();
    all_words.sort_unstable(); // Sort for deterministic error messages.
    for word in all_words {
        let input_count = input_hist.get(word).copied().unwrap_or(0);
        let output_count = output_hist.get(word).copied().unwrap_or(0);
        assert_eq!(input_count, output_count, "Different word count for \"{word}\" INPUT {input_count} OUTPUT {output_count}");
    }
}

/// Make a histogram of word occurrences.
/// Also include angle brackets and slashes.
/// Do not include numbers, since rust may format them differently (with or
/// without scientific notation).
fn xml_histogram(xml_data: &str) -> HashMap<String, u64> {
    let mut hist = HashMap::new();
    let split_re = regex::Regex::new(r"(\w+)").unwrap();
    let number_re = regex::Regex::new(r"\A[\d]+.*").unwrap();
    for word in split_re.find_iter(&xml_data).map(|c| c.as_str().to_owned()) {
        if number_re.is_match(&word) {
            continue;
        }
        hist.entry(word).and_modify(|x| *x += 1).or_insert(1);
    }
    hist
}

/// Remove all comments from the XML.
/// Comments look like:
///     <!--  Hello XML  -->
fn strip_comments(xml_data: &str) -> String {
    let comment_re = regex::Regex::new(r"(?s)(<!--.*?-->)").unwrap();
    return comment_re.replace_all(xml_data, "").into();
}

/// Remove any closing tags that do not contain anything.
///
/// For example:
///     <tag attr="5"> </tag>
///
/// Should become just:
///     <tag attr="5"/>
///
fn strip_empty_tags(xml_data: &str) -> String {
    let empty_tag_re = regex::Regex::new(r"(?s)<(\w+)([^/>]*?)>\s*</(\w+)>").unwrap();
    let replacement = |caps: &regex::Captures| -> String {
        let start = &caps[1];
        let attrs = &caps[2];
        let end = &caps[3];
        if start == end {
            format!("<{start} {attrs}/>")
        } else {
            caps[0].to_owned()
        }
    };
    return empty_tag_re.replace_all(xml_data, replacement).into();
}

/// Remove any tag that looks like:
///     <?xml version="1.0" encoding="UTF-8"?>
fn strip_xml_header(xml_data: &str) -> String {
    let xml_header_re = regex::Regex::new(r"(?s)(<\?xml(\s?.*?)\?>)").unwrap();
    return xml_header_re.replace_all(xml_data, "").into();
}

fn check_joint_order(input: &str, output: &str) {
    let input_order = get_joint_order(input);
    let output_order = get_joint_order(output);
    assert_eq!(input_order, output_order);
}

fn get_joint_order(xml_data: &str) -> Vec<String> {
    todo!()
}

//
// MUJOCO MENAGERIE
//

#[test]
fn agility_cassie_cassie() {
    test_mjcf("mujoco_menagerie/agility_cassie/cassie.xml")
}
#[test]
fn agility_cassie_scene() {
    test_mjcf("mujoco_menagerie/agility_cassie/scene.xml")
}
#[test]
fn anybotics_anymal_b_anymal_b() {
    test_mjcf("mujoco_menagerie/anybotics_anymal_b/anymal_b.xml")
}
#[test]
fn anybotics_anymal_b_scene() {
    test_mjcf("mujoco_menagerie/anybotics_anymal_b/scene.xml")
}
#[test]
fn anybotics_anymal_c_anymal_c() {
    test_mjcf("mujoco_menagerie/anybotics_anymal_c/anymal_c.xml")
}
#[test]
fn anybotics_anymal_c_scene() {
    test_mjcf("mujoco_menagerie/anybotics_anymal_c/scene.xml")
}
#[test]
fn franka_emika_panda_hand() {
    test_mjcf("mujoco_menagerie/franka_emika_panda/hand.xml")
}
#[test]
fn franka_emika_panda_panda() {
    test_mjcf("mujoco_menagerie/franka_emika_panda/panda.xml")
}
#[test]
fn franka_emika_panda_panda_nohand() {
    test_mjcf("mujoco_menagerie/franka_emika_panda/panda_nohand.xml")
}
#[test]
fn franka_emika_panda_scene() {
    test_mjcf("mujoco_menagerie/franka_emika_panda/scene.xml")
}
#[test]
fn google_barkour_v0_barkour_v0() {
    test_mjcf("mujoco_menagerie/google_barkour_v0/barkour_v0.xml")
}
#[test]
fn google_barkour_v0_scene() {
    test_mjcf("mujoco_menagerie/google_barkour_v0/scene.xml")
}
#[test]
fn google_barkour_v0_scene_barkour() {
    test_mjcf("mujoco_menagerie/google_barkour_v0/scene_barkour.xml")
}
#[test]
fn google_barkour_vb_barkour_vb() {
    test_mjcf("mujoco_menagerie/google_barkour_vb/barkour_vb.xml")
}
#[test]
fn google_barkour_vb_barkour_mjx() {
    test_mjcf("mujoco_menagerie/google_barkour_vb/barkour_vb_mjx.xml")
}
#[test]
fn google_barkour_vb_scene() {
    test_mjcf("mujoco_menagerie/google_barkour_vb/scene.xml")
}
#[test]
fn google_barkour_vb_scene_mjx() {
    test_mjcf("mujoco_menagerie/google_barkour_vb/scene_mjx.xml")
}
#[test]
fn google_robot_robot() {
    test_mjcf("mujoco_menagerie/google_robot/robot.xml")
}
#[test]
fn google_robot_scene() {
    test_mjcf("mujoco_menagerie/google_robot/scene.xml")
}
#[test]
fn hello_robot_stretch_scene() {
    test_mjcf("mujoco_menagerie/hello_robot_stretch/scene.xml")
}
#[test]
fn hello_robot_stretch_stretch() {
    test_mjcf("mujoco_menagerie/hello_robot_stretch/stretch.xml")
}
#[test]
fn kuka_iiwa_14_iiwa14() {
    test_mjcf("mujoco_menagerie/kuka_iiwa_14/iiwa14.xml")
}
#[test]
fn kuka_iiwa_14_scene() {
    test_mjcf("mujoco_menagerie/kuka_iiwa_14/scene.xml")
}
#[test]
fn realsense_d435i_d435i() {
    test_mjcf("mujoco_menagerie/realsense_d435i/d435i.xml")
}
#[test]
fn rethink_robotics_sawyer_sawyer() {
    test_mjcf("mujoco_menagerie/rethink_robotics_sawyer/sawyer.xml")
}
#[test]
fn rethink_robotics_sawyer_scene() {
    test_mjcf("mujoco_menagerie/rethink_robotics_sawyer/scene.xml")
}
#[test]
fn robotiq_2f85_2f85() {
    test_mjcf("mujoco_menagerie/robotiq_2f85/2f85.xml")
}
#[test]
fn robotiq_2f85_scene() {
    test_mjcf("mujoco_menagerie/robotiq_2f85/scene.xml")
}
#[test]
fn robotis_op3_op3() {
    test_mjcf("mujoco_menagerie/robotis_op3/op3.xml")
}
#[test]
fn robotis_op3_scene() {
    test_mjcf("mujoco_menagerie/robotis_op3/scene.xml")
}
#[test]
fn shadow_hand_keyframes() {
    test_mjcf("mujoco_menagerie/shadow_hand/keyframes.xml")
}
#[test]
fn shadow_hand_left_hand() {
    test_mjcf("mujoco_menagerie/shadow_hand/left_hand.xml")
}
#[test]
fn shadow_hand_right_hand() {
    test_mjcf("mujoco_menagerie/shadow_hand/right_hand.xml")
}
#[test]
fn shadow_hand_scene_left() {
    test_mjcf("mujoco_menagerie/shadow_hand/scene_left.xml")
}
#[test]
fn shadow_hand_scene_right() {
    test_mjcf("mujoco_menagerie/shadow_hand/scene_right.xml")
}
#[test]
fn skydio_x2_scene() {
    test_mjcf("mujoco_menagerie/skydio_x2/scene.xml")
}
#[test]
fn skydio_x2_x2() {
    test_mjcf("mujoco_menagerie/skydio_x2/x2.xml")
}
#[test]
fn trossen_vx300s_scene() {
    test_mjcf("mujoco_menagerie/trossen_vx300s/scene.xml")
}
#[test]
fn trossen_vx300s_vx300s() {
    test_mjcf("mujoco_menagerie/trossen_vx300s/vx300s.xml")
}
#[test]
fn ufactory_lite6_lite6() {
    test_mjcf("mujoco_menagerie/ufactory_lite6/lite6.xml")
}
#[test]
fn ufactory_lite6_scene() {
    test_mjcf("mujoco_menagerie/ufactory_lite6/scene.xml")
}
#[test]
fn ufactory_xarm7_hand() {
    test_mjcf("mujoco_menagerie/ufactory_xarm7/hand.xml")
}
#[test]
fn ufactory_xarm7_scene() {
    test_mjcf("mujoco_menagerie/ufactory_xarm7/scene.xml")
}
#[test]
fn ufactory_xarm7_xarm7() {
    test_mjcf("mujoco_menagerie/ufactory_xarm7/xarm7.xml")
}
#[test]
fn ufactory_xarm7_xarm7_nohand() {
    test_mjcf("mujoco_menagerie/ufactory_xarm7/xarm7_nohand.xml")
}
#[test]
fn unitree_a1_a1() {
    test_mjcf("mujoco_menagerie/unitree_a1/a1.xml")
}
#[test]
fn unitree_a1_scene() {
    test_mjcf("mujoco_menagerie/unitree_a1/scene.xml")
}
#[test]
fn unitree_go1_go1() {
    test_mjcf("mujoco_menagerie/unitree_go1/go1.xml")
}
#[test]
fn unitree_go1_scene() {
    test_mjcf("mujoco_menagerie/unitree_go1/scene.xml")
}
#[test]
fn unitree_go2_go2() {
    test_mjcf("mujoco_menagerie/unitree_go2/go2.xml")
}
#[test]
fn unitree_go2_scene() {
    test_mjcf("mujoco_menagerie/unitree_go2/scene.xml")
}
#[test]
fn unitree_h1_h1() {
    test_mjcf("mujoco_menagerie/unitree_h1/h1.xml")
}
#[test]
fn unitree_h1_scene() {
    test_mjcf("mujoco_menagerie/unitree_h1/scene.xml")
}
#[test]
fn unitree_z1_scene() {
    test_mjcf("mujoco_menagerie/unitree_z1/scene.xml")
}
#[test]
fn unitree_z1_z1() {
    test_mjcf("mujoco_menagerie/unitree_z1/z1.xml")
}
#[test]
fn universal_robots_ur10e_scene() {
    test_mjcf("mujoco_menagerie/universal_robots_ur10e/scene.xml")
}
#[test]
fn universal_robots_ur10e_ur10e() {
    test_mjcf("mujoco_menagerie/universal_robots_ur10e/ur10e.xml")
}
#[test]
fn universal_robots_ur5e_scene() {
    test_mjcf("mujoco_menagerie/universal_robots_ur5e/scene.xml")
}
#[test]
fn universal_robots_ur5e_ur5e() {
    test_mjcf("mujoco_menagerie/universal_robots_ur5e/ur5e.xml")
}
#[test]
fn wonik_allegro_left_hand() {
    test_mjcf("mujoco_menagerie/wonik_allegro/left_hand.xml")
}
#[test]
fn wonik_allegro_right_hand() {
    test_mjcf("mujoco_menagerie/wonik_allegro/right_hand.xml")
}
#[test]
fn wonik_allegro_scene_left() {
    test_mjcf("mujoco_menagerie/wonik_allegro/scene_left.xml")
}
#[test]
fn wonik_allegro_scene_right() {
    test_mjcf("mujoco_menagerie/wonik_allegro/scene_right.xml")
}

//
// GYMNASIUM
//

#[test]
fn gymnasium_ant() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/ant.xml")
}
#[test]
fn gymnasium_half_cheetah() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/half_cheetah.xml")
}
#[test]
fn gymnasium_hopper() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/hopper.xml")
}
#[test]
fn gymnasium_humanoidstandup() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/humanoidstandup.xml")
}
#[test]
fn gymnasium_humanoid() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/humanoid.xml")
}
#[test]
fn gymnasium_inverted_double_pendulum() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/inverted_double_pendulum.xml")
}
#[test]
fn gymnasium_inverted_pendulum() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/inverted_pendulum.xml")
}
#[test]
fn gymnasium_point() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/point.xml")
}
#[test]
fn gymnasium_pusher() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/pusher.xml")
}
#[test]
fn gymnasium_reacher() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/reacher.xml")
}
#[test]
fn gymnasium_swimmer() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/swimmer.xml")
}
#[test]
fn gymnasium_walker2d_v5() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/walker2d_v5.xml")
}
#[test]
fn gymnasium_walker2d() {
    test_mjcf("Gymnasium/gymnasium/envs/mujoco/assets/walker2d.xml")
}
#[test]
fn gymnasium_walker2d_v5_uneven_feet() {
    test_mjcf("Gymnasium/tests/envs/mujoco/assets/walker2d_v5_uneven_feet.xml")
}

//
// DM_CONTROL
//

#[test]
fn composer_arena() {
    test_mjcf("dm_control/dm_control/composer/arena.xml")
}
#[test]
fn duplo_duplo2x4() {
    test_mjcf("dm_control/dm_control/entities/props/duplo/duplo2x4.xml")
}
#[test]
fn boxhead_boxhead() {
    test_mjcf("dm_control/dm_control/locomotion/soccer/assets/boxhead/boxhead.xml")
}
#[test]
fn assets_drosophila_defaults() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/build_fruitfly/assets/drosophila_defaults.xml")
}
#[test]
fn assets_drosophila_fused() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/build_fruitfly/assets/drosophila_fused.xml")
}
#[test]
fn assets_drosophila() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/build_fruitfly/assets/drosophila.xml")
}
#[test]
fn build_fruitfly_floor() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/build_fruitfly/floor.xml")
}
#[test]
fn build_fruitfly_fruitfly() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/build_fruitfly/fruitfly.xml")
}
#[test]
fn dog_v2_dog_base() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/dog_v2/dog_base.xml")
}
#[test]
fn dog_v2_dog() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/dog_v2/dog.xml")
}
#[test]
fn dog_v2_scene() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/dog_v2/scene.xml")
}
#[test]
fn fruitfly_v2_floor() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/fruitfly_v2/floor.xml")
}
#[test]
fn fruitfly_v2_fruitfly() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/fruitfly_v2/fruitfly.xml")
}
#[test]
fn assets_humanoid_CMU_V2019() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/humanoid_CMU_V2019.xml")
}
#[test]
fn assets_humanoid_CMU_V2020() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/humanoid_CMU_V2020.xml")
}
#[test]
fn jumping_ball_jumping_ball_with_head() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/jumping_ball/jumping_ball_with_head.xml")
}
#[test]
fn assets_rodent() {
    test_mjcf("dm_control/dm_control/locomotion/walkers/assets/rodent.xml")
}
#[test]
fn test_assets_arena() {
    test_mjcf("dm_control/dm_control/mjcf/test_assets/arena.xml")
}
#[test]
fn test_assets_included_with_invalid_filenames() {
    test_mjcf("dm_control/dm_control/mjcf/test_assets/included_with_invalid_filenames.xml")
}
#[test]
fn test_assets_included() {
    test_mjcf("dm_control/dm_control/mjcf/test_assets/included.xml")
}
#[test]
fn test_assets_lego_brick() {
    test_mjcf("dm_control/dm_control/mjcf/test_assets/lego_brick.xml")
}
#[test]
fn test_assets_model_with_assets() {
    test_mjcf("dm_control/dm_control/mjcf/test_assets/model_with_assets.xml")
}
#[test]
fn test_assets_model_with_include() {
    test_mjcf("dm_control/dm_control/mjcf/test_assets/model_with_include.xml")
}
#[test]
fn test_assets_model_with_invalid_filenames() {
    test_mjcf("dm_control/dm_control/mjcf/test_assets/model_with_invalid_filenames.xml")
}
#[test]
fn test_assets_model_with_nameless_assets() {
    test_mjcf("dm_control/dm_control/mjcf/test_assets/model_with_nameless_assets.xml")
}
#[test]
fn test_assets_robot_arm() {
    test_mjcf("dm_control/dm_control/mjcf/test_assets/robot_arm.xml")
}
#[test]
fn test_assets_test_model() {
    test_mjcf("dm_control/dm_control/mjcf/test_assets/test_model.xml")
}
#[test]
fn assets_arm() {
    test_mjcf("dm_control/dm_control/mujoco/testing/assets/arm.xml")
}
#[test]
fn assets_cartpole_no_names() {
    test_mjcf("dm_control/dm_control/mujoco/testing/assets/cartpole_no_names.xml")
}
#[test]
fn assets_cartpole() {
    test_mjcf("dm_control/dm_control/mujoco/testing/assets/cartpole.xml")
}
#[test]
fn assets_humanoid() {
    test_mjcf("dm_control/dm_control/mujoco/testing/assets/humanoid.xml")
}
#[test]
fn assets_model_with_assets() {
    test_mjcf("dm_control/dm_control/mujoco/testing/assets/model_with_assets.xml")
}
#[test]
fn assets_model_with_ball_joints() {
    test_mjcf("dm_control/dm_control/mujoco/testing/assets/model_with_ball_joints.xml")
}
#[test]
fn assets_model_with_third_order_actuators() {
    test_mjcf("dm_control/dm_control/mujoco/testing/assets/model_with_third_order_actuators.xml")
}
#[test]
fn assets_sphere() {
    test_mjcf("dm_control/dm_control/mujoco/testing/assets/sphere.xml")
}
#[test]
fn suite_acrobot() {
    test_mjcf("dm_control/dm_control/suite/acrobot.xml")
}
#[test]
fn suite_ball_in_cup() {
    test_mjcf("dm_control/dm_control/suite/ball_in_cup.xml")
}
#[test]
fn suite_cartpole() {
    test_mjcf("dm_control/dm_control/suite/cartpole.xml")
}
#[test]
fn suite_cheetah() {
    test_mjcf("dm_control/dm_control/suite/cheetah.xml")
}
#[test]
fn common_materials() {
    test_mjcf("dm_control/dm_control/suite/common/materials.xml")
}
#[test]
fn common_skybox() {
    test_mjcf("dm_control/dm_control/suite/common/skybox.xml")
}
#[test]
fn common_visual() {
    test_mjcf("dm_control/dm_control/suite/common/visual.xml")
}
#[test]
fn suite_dog() {
    test_mjcf("dm_control/dm_control/suite/dog.xml")
}
#[test]
fn suite_finger() {
    test_mjcf("dm_control/dm_control/suite/finger.xml")
}
#[test]
fn suite_fish() {
    test_mjcf("dm_control/dm_control/suite/fish.xml")
}
#[test]
fn suite_hopper() {
    test_mjcf("dm_control/dm_control/suite/hopper.xml")
}
#[test]
fn suite_humanoid_CMU() {
    test_mjcf("dm_control/dm_control/suite/humanoid_CMU.xml")
}
#[test]
fn suite_humanoid() {
    test_mjcf("dm_control/dm_control/suite/humanoid.xml")
}
#[test]
fn suite_lqr() {
    test_mjcf("dm_control/dm_control/suite/lqr.xml")
}
#[test]
fn suite_manipulator() {
    test_mjcf("dm_control/dm_control/suite/manipulator.xml")
}
#[test]
fn suite_pendulum() {
    test_mjcf("dm_control/dm_control/suite/pendulum.xml")
}
#[test]
fn suite_point_mass() {
    test_mjcf("dm_control/dm_control/suite/point_mass.xml")
}
#[test]
fn suite_quadruped() {
    test_mjcf("dm_control/dm_control/suite/quadruped.xml")
}
#[test]
fn suite_reacher() {
    test_mjcf("dm_control/dm_control/suite/reacher.xml")
}
#[test]
fn suite_stacker() {
    test_mjcf("dm_control/dm_control/suite/stacker.xml")
}
#[test]
fn suite_swimmer() {
    test_mjcf("dm_control/dm_control/suite/swimmer.xml")
}
#[test]
fn suite_walker() {
    test_mjcf("dm_control/dm_control/suite/walker.xml")
}
#[test]
fn ant_ant() {
    test_mjcf("dm_control/dm_control/third_party/ant/ant.xml")
}
#[test]
fn kinova_common() {
    test_mjcf("dm_control/dm_control/third_party/kinova/common.xml")
}
#[test]
fn kinova_jaco_arm() {
    test_mjcf("dm_control/dm_control/third_party/kinova/jaco_arm.xml")
}
#[test]
fn kinova_jaco_hand() {
    test_mjcf("dm_control/dm_control/third_party/kinova/jaco_hand.xml")
}
