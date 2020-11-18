pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|win| format!("For want of a {} the {} was lost.", win[0], win[1]))
        .chain(
            list.first()
                .map(|x| format!("And all for the want of a {}.", x)),
        )
        .collect::<Vec<String>>()
        .join("\n")
    // let mut proverb: Vec<String> = vec![];

    // for (i, _) in list.iter().enumerate() {
    //     if i == list.len() - 1 {
    //         proverb.push(format!("And all for the want of a {}.", list[0],));
    //     } else {
    //         proverb.push(format!(
    //             "For want of a {} the {} was lost.",
    //             list[i],
    //             list[i + 1]
    //         ));
    //     }
    // }
    // proverb.join("\n")
}
