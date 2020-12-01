// day 1  - https://adventofcode.com/2020/day/1
// PART 1

// return true if sum is 2020
fn is_sum_of_two_2020(a : i32, b : i32) -> bool {
    a + b == 2020
}

// find a pair in  array that sums up to 2020
fn find_sum_of_two(arr: &[i32]) -> Option<(i32, i32)> {
    if arr.is_empty() {
        // done
        None
    } else {
        // car cdr
        let (first, rest) = (arr[0], &arr[1..arr.len()]);

        // search for a pair where first + any of the rest = 2020
        let found = rest.iter().find(|&&x| is_sum_of_two_2020(first, x));

        match found {
            // found a match, return it
            Some(&x) => Some((first, x)),

            // didn't find a match, move on to the next number in array
            None => find_sum_of_two(rest)
        }
    }
}


#[test]
fn test_is_sum_of_two_2020() {
    // happy 😺
    assert_eq!(is_sum_of_two_2020(2019, 1), true);
    assert_eq!(is_sum_of_two_2020(1010, 1010), true);

    // sad 😞
    assert_eq!(is_sum_of_two_2020(2020, 1), false);
    assert_eq!(is_sum_of_two_2020(1010, 1011), false);
}


#[test]
fn test_find_sum_of_two() {
    // empty array should return None
    let empty: [i32; 0] = [];
    assert_eq!(find_sum_of_two(&empty), None);

    // not found
    let one: [i32; 1] = [1];
    assert_eq!(find_sum_of_two(&one), None);

    // not found
    let two: [i32; 2] = [1, 2];
    assert_eq!(find_sum_of_two(&two), None);

    // one not found
    let one_found: [i32; 1] = [2020];
    assert_eq!(find_sum_of_two(&one_found), None);

    // two found
    let two_found: [i32; 2] = [1010, 1010];
    assert_eq!(find_sum_of_two(&two_found), Some((1010, 1010)));

    // two different found
    let two_different_found: [i32; 2] = [1000, 1020];
    assert_eq!(find_sum_of_two(&two_different_found), Some((1000, 1020)));

    // test data
    let test_data: [i32; 6] = [1721, 979, 366, 299, 675, 1456];
    assert_eq!(find_sum_of_two(&test_data), Some((1721, 299)));
}

// PART 2

/**
 * Find {size} numbers in {input} that sums up to {sum}.
 * input (&[i32]) - Array where we're looking for numbers
 * sum (i32) - Numbers should add up to this sum
 * size (i32) - How many numbers to combine to get the sum
 */
fn find(input: &[i32], sum: i32, size: i32) -> Option<Vec<i32>> {
    if sum == 0 && size == 0 {
        // found the number - return an empty vector
        Some(vec![])
    } else if sum < 0 || size <= 0 {
        // number was not found
        None
    } 
    else {
        let mut n = 0;
        loop { // -> Option<Vec<i32>>
            if n >= input.len() {
                break None;
            } else {
                // recurse
                match find(&input[n..], sum - input[n], size - 1) {
                    Some(mut v) => {
                        // concat input[n] with the result
                        let mut result = vec![input[n]];
                        result.append(&mut v);
                        break Some(result);
                    },
                    None => {
                        // increment n
                        n += 1;
                        continue;
                    }
                }
            }
        }
    }
}

#[test]
fn test_find() {
    // not found if sum is negative
    let empty: [i32; 0] = [];
    assert_eq!(find(&empty, -1, 1), None);

    // not found if size is zero
    let empty: [i32; 0] = [];
    assert_eq!(find(&empty, 200, 0), None);

    // not found if input is empty
    let empty: [i32; 0] = [];
    assert_eq!(find(&empty, 200, 2), None);

    // test data with two
    let test_data: [i32; 6] = [1721, 979, 366, 299, 675, 1456];
    assert_eq!(find(&test_data, 2020, 2), Some(vec![1721, 299]));

    // test data with three
    let test_data: [i32; 6] = [1721, 979, 366, 299, 675, 1456];
    assert_eq!(find(&test_data, 2020, 3), Some(vec![979, 366, 675]));
}


fn main() {
    let expenses: [i32; 200] = [1825, 1944, 1802, 1676, 1921, 1652, 1710, 1952, 1932, 1934, 1823, 1732, 1795, 1681, 1706, 1697, 1919, 1695, 2007, 1889, 1942, 961, 1868, 1878, 1723, 416, 1875, 1831, 1890, 1654, 1956, 1827, 973, 1947, 1688, 1680, 1808, 1998, 1794, 1552, 1935, 1693, 1824, 1711, 1766, 1668, 1968, 1884, 217, 2003, 1869, 1658, 1953, 1829, 1984, 2005, 1973, 428, 1957, 1925, 1719, 1797, 321, 1804, 1971, 922, 1976, 1863, 2008, 1806, 1833, 1809, 1707, 1954, 1811, 1815, 1915, 1799, 1917, 1664, 1937, 1775, 1685, 1756, 1940, 1660, 1859, 1916, 1989, 1763, 1994, 1716, 1689, 1866, 1708, 1670, 1982, 1870, 1847, 1627, 1819, 1786, 1828, 1640, 1699, 1722, 1737, 1882, 1666, 1871, 1703, 1770, 1623, 1837, 1636, 1655, 1930, 1739, 1810, 1805, 1861, 1922, 1993, 1896, 1760, 2002, 1779, 1633, 1972, 1856, 1641, 1718, 2004, 1730, 1826, 1923, 1753, 1735, 660, 1988, 1796, 1990, 1720, 1626, 1788, 1700, 942, 1902, 1943, 1758, 1839, 1924, 938, 1634, 1724, 1983, 1683, 1687, 1904, 1907, 1757, 2001, 1910, 1849, 1781, 1981, 1743, 1851, 2009, 619, 1898, 1891, 1751, 1765, 1959, 1888, 1894, 1759, 389, 1964, 1900, 1742, 1672, 1969, 1978, 1933, 1906, 1807, 1867, 1838, 1960, 1814, 1950, 1918, 1726, 1986, 1746, 2006, 1949, 1784];
    
    match find_sum_of_two(&expenses) {
        // found pair a + b = 2020
        Some((a, b)) => println!("{} + {} = {}, {} * {} = {}", a, b, a + b, a, b, a * b),
        // didn't find a pair
        None => println!("No match was found")
    }

    match find(&expenses, 2020, 3) {
        // found a triplet a + b + c = 2020
        Some(v) => println!("{} + {} + {} = {}, {} * {} * {} = {}", v[0], v[1], v[2], v[0] + v[1] + v[2], v[0], v[1], v[2], v[0] * v[1] * v[2]),
        // didn't find a triplet
        None => println!("No match was found")
    }
}
