use rust_counter::app::main;

fn main() {
    let document = gloo::utils::document();
    let body = document.query_selector("body").unwrap().unwrap();

    let mount_point = document.create_element("div").unwrap();
    let class_list = mount_point.class_list();
    class_list.add_1("root").unwrap();

    body.append_child(&mount_point).unwrap();

    yew::Renderer::<main::App>::with_root(mount_point).render();
}
