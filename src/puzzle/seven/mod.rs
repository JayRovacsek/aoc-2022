use regex::Regex;

mod test;

pub fn solve(input: &str) {
    let answer_part_one = solve_part_one(&input);
    let answer_part_two = solve_part_two(&input);
    println!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

#[derive(Debug, Clone)]
struct Inode {
    children: Vec<Inode>,
    file: bool,
    size: usize,
    name: String,
}

impl Inode {
    fn size(&self) -> usize {
        match self.file {
            true => self.size,
            false => self.children.iter().fold(0, |acc, x| acc + x.size()),
        }
    }

    fn parse_root(input: &str) -> Self {
        let raw_nodes = input
            .split("$ cd ")
            .into_iter()
            .map(|x| {
                x.split("\n")
                    .map(|x| String::from(x))
                    .collect::<Vec<String>>()
            })
            .filter(|x| x.len() > 2)
            .collect::<Vec<Vec<String>>>();

        let children = Inode::parse_recursive(raw_nodes, String::from("/"));

        let size = children.iter().fold(0, |acc, x| x.size() + acc);

        Inode {
            children,
            file: false,
            size,
            name: String::from("/"),
        }
    }

    fn parse_recursive(input: Vec<Vec<String>>, name: String) -> Vec<Inode> {
        let re = Regex::new(r"^(dir|\d)").unwrap();
        let file = Regex::new(r"^\d").unwrap();

        input
            .iter()
            .filter(|x| *x.first().unwrap() == name)
            .map(|x| {
                x.iter()
                    .filter(|y| re.is_match(y) && **y != name)
                    .map(|y| match file.is_match(y) {
                        true => {
                            let parts = y.split(" ").collect::<Vec<&str>>();
                            let file_name = String::from(*parts.last().unwrap());
                            let size = parts.first().unwrap().parse::<usize>().unwrap_or(0_usize);
                            Inode {
                                name: file_name,
                                size,
                                file: true,
                                children: vec![],
                            }
                        }
                        false => {
                            let parts = y.split(" ").collect::<Vec<&str>>();
                            let child_name = String::from(*parts.last().unwrap());
                            let children =
                                Inode::parse_recursive(input.clone(), child_name.clone());

                            let size = children.iter().fold(0, |acc, x| x.size() + acc);

                            Inode {
                                name: child_name,
                                size,
                                file: false,
                                children,
                            }
                        }
                    })
                    .collect::<Vec<Inode>>()
            })
            .flatten()
            .collect::<Vec<Inode>>()
    }

    fn filter_folder_size_recursive(&self, size: usize) -> Vec<Inode> {
        let suitable_children = self
            .children
            .iter()
            .filter(|x| x.file == false && x.size < size)
            .map(|x| Inode {
                name: x.name.clone(),
                size: x.size,
                file: x.file,
                children: vec![],
            });

        self.children
            .iter()
            .filter(|x| x.file == false)
            .map(|x| x.filter_folder_size_recursive(size))
            .flatten()
            .chain(suitable_children)
            .collect::<Vec<Inode>>()
    }
}

pub fn solve_part_one(input: &str) -> String {
    let root = Inode::parse_root(input);

    let suitable_size_inodes = root.filter_folder_size_recursive(100000);

    let sum = suitable_size_inodes.iter().fold(0, |acc, x| acc + x.size);

    format!("{}", sum)
}

pub fn solve_part_two(input: &str) -> String {
    String::from("")
}
