const ELEMENTS: [&'static str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-layin'",
    "Seven swans a-swimmin'",
    "Eight maids milkin'",
    "Nine lords a-leapin'",
    "Ten ladies dancin'",
    "Eleven pipers pipin'",
    "Twelve drummers drummin'",
];

fn get_element(n: usize) -> &'static str {
    ELEMENTS[n]
}

pub fn append_element(index: usize) -> String {
    let mut phrase = String::new();
    for i in (0..index + 1).rev() {
        if i == 1 {
            phrase = phrase + " " + ELEMENTS[i as usize] + " and";
        } else if i != 0 {
            phrase = phrase + " " + ELEMENTS[i as usize] + ",";
        } else {
            phrase = phrase + " " + ELEMENTS[i as usize];
        }
    }
    phrase
}
