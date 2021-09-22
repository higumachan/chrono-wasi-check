use chrono::prelude::*;


fn get_another_epoch<T>(timezone: &T) -> Date<T>
where
    T: TimeZone,
{
        timezone.ymd(1969, 1, 1)
}

fn get_unix_epoch<T>(timezone: &T) -> Date<T>
where
    T: TimeZone,
{
        timezone.ymd(1970, 1, 1)
}

fn main() {
    println!("utc       ={:?}", get_unix_epoch(&Utc));
    println!("fixed+9   ={:?}", get_unix_epoch(&FixedOffset::east(9 * 3600)));
    println!("fixed+0   ={:?}", get_unix_epoch(&FixedOffset::east(0)));
    println!("local     ={:?}", get_unix_epoch(&Local));
    println!("utc       ={:?}", get_another_epoch(&Utc));
    println!("fixed+9   ={:?}", get_another_epoch(&FixedOffset::east(9 * 3600)));
    println!("fixed+0   ={:?}", get_another_epoch(&FixedOffset::east(0)));
    println!("local     ={:?}", get_another_epoch(&Local));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn get_epoch_local() {
        let ep = get_epoch(&Local);
    }

    #[test]
    fn get_epoch_fixed_offset_0() {
        let ep = get_epoch(&FixedOffset::east(0));
    }

    #[test]
    fn get_epoch_utc() {
        let ep = get_epoch(&Utc);
    }
}
