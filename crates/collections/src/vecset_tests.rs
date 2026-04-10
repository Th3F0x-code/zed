use super::vecset::VecSet;

#[test]
fn test_insert_and_contains() {
    let mut set = VecSet::new();
    assert!(set.insert(1));
    assert!(set.insert(2));
    assert!(!set.insert(1));
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(!set.contains(&3));
    assert_eq!(set.len(), 2);
}

#[test]
fn test_remove() {
    let mut set = VecSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    assert!(set.remove(&2));
    assert!(!set.remove(&2));
    assert!(!set.contains(&2));
    assert_eq!(set.len(), 2);
}

#[test]
fn test_iteration_order() {
    let mut set = VecSet::new();
    set.insert("c");
    set.insert("a");
    set.insert("b");
    let items: Vec<_> = set.iter().copied().collect();
    assert_eq!(items, vec!["c", "a", "b"]);
}

#[test]
fn test_into_iter() {
    let mut set = VecSet::new();
    set.insert(10);
    set.insert(20);
    let items: Vec<_> = set.into_iter().collect();
    assert_eq!(items, vec![10, 20]);
}

#[test]
fn test_retain() {
    let mut set = VecSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.retain(|x| x % 2 == 0);
    assert_eq!(set.len(), 2);
    assert!(set.contains(&2));
    assert!(set.contains(&4));
}
