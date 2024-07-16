use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Rc<RefCell<(String, HashSet<String>)>>> = HashMap::new();
    let mut ppls = vec![];

    for account in accounts {
        let mut ppl: Rc<RefCell<(String, HashSet<String>)>> =
            Rc::new(RefCell::new(("".to_string(), HashSet::new())));
        for email in &account[1..] {
            if map.get(email).is_some() {
                let v = map.get(email).unwrap().clone();
                if ppl.borrow().0 == "".to_string() {
                    ppl = v.clone();
                } else if ppl != v {
                    for sub_mail in &v.borrow_mut().1 {
                        ppl.borrow_mut().1.insert(sub_mail.clone());
                        map.insert(sub_mail.clone(), ppl.clone());
                    }
                    ppls.retain(|x| x != &v);
                }
            }
        }

        if ppl.borrow().0 == "".to_string() {
            ppl = Rc::new(RefCell::new((account[0].clone(), HashSet::new())));
            ppls.push(ppl.clone());
        }

        for email in account.into_iter().skip(1) {
            map.insert(email.clone(), ppl.clone());
            ppl.borrow_mut().1.insert(email);
        }
    }

    ppls.into_iter()
        .filter(|x| x.borrow().1.len() > 0)
        .map(|x| {
            let ppl = x.take();
            let mut ret = vec![ppl.0];
            let mut emails = ppl.1.into_iter().collect::<Vec<_>>();
            emails.sort();
            ret.extend(emails);
            ret
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        let accounts = vec![
            vec_string!["John", "johnsmith@mail.com", "john00@mail.com"],
            vec_string!["John", "johnnybravo@mail.com"],
            vec_string!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            vec_string!["Mary", "mary@mail.com"],
        ];
        assert_eq!(
            accounts_merge(accounts),
            vec![
                vec_string![
                    "John",
                    "john00@mail.com",
                    "john_newyork@mail.com",
                    "johnsmith@mail.com",
                ],
                vec_string!["John", "johnnybravo@mail.com"],
                vec_string!["Mary", "mary@mail.com"],
            ]
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            accounts_merge(vec![
                vec_string!["Hanzo", "Hanzo2@m.co", "Hanzo3@m.co"],
                vec_string!["Hanzo", "Hanzo4@m.co", "Hanzo5@m.co"],
                vec_string!["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co"],
                vec_string!["Hanzo", "Hanzo3@m.co", "Hanzo4@m.co"],
                vec_string!["Hanzo", "Hanzo7@m.co", "Hanzo8@m.co"],
                vec_string!["Hanzo", "Hanzo1@m.co", "Hanzo2@m.co"],
                vec_string!["Hanzo", "Hanzo6@m.co", "Hanzo7@m.co"],
                vec_string!["Hanzo", "Hanzo5@m.co", "Hanzo6@m.co"],
            ]),
            vec![vec_string![
                "Hanzo",
                "Hanzo0@m.co",
                "Hanzo1@m.co",
                "Hanzo2@m.co",
                "Hanzo3@m.co",
                "Hanzo4@m.co",
                "Hanzo5@m.co",
                "Hanzo6@m.co",
                "Hanzo7@m.co",
                "Hanzo8@m.co"
            ]]
        );
    }
}
