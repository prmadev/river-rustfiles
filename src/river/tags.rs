use super::ctl;

pub async fn tags() {
    static MAP: &str = "map";
    static NORMAL: &str = "normal";
    static SET_FOCUS: &str = "set-focused-tags";
    static TOGGLE_FOCUS: &str = "toggle-focused-tags";
    static TOGGLE_VIEW: &str = "toggle-view-tags";
    static SET_VIEW: &str = "set-view-tags";

    for i in 1..=9 {
        let numb = format!("{}", i);
        let tag = format!("{}", 1 << (i - 1));

        let que: Vec<Vec<&str>> = vec![
            vec![MAP, NORMAL, "Super", &numb, SET_FOCUS, &tag],
            vec![MAP, NORMAL, "Super+Shift", &numb, SET_VIEW, &tag],
            vec![MAP, NORMAL, "Super+Control", &numb, TOGGLE_FOCUS, &tag],
            vec![MAP, NORMAL, "Super+Shift+Control", &numb, TOGGLE_VIEW, &tag],
        ];

        let mut handles = vec![];
        for command in que.iter() {
            handles.push(ctl(command.to_vec()));
        }
        for handle in handles {
            handle.await;
        }
    }

    let all_tags = format!("{}", (1u64 << 32) - 1);
    ctl(vec![MAP, NORMAL, "Super", "0", SET_FOCUS, &all_tags]).await;
    ctl(vec![MAP, NORMAL, "Super+Shift", "0", SET_VIEW, &all_tags]).await;
}
