use chrono::prelude::*;
//use chrono::Local;
use uuid::Uuid;

pub(crate) fn as_uuid(s: String) -> Uuid {
    let mut reconstructed = String::new();
    let s = if s.starts_with('{') && s.ends_with('}') {
        // reconstruct the string without the braces
        for c in s.chars().skip(1).take(s.len() - 2) {
            reconstructed.push(c);
        }

        reconstructed.as_str()
    } else {
        s.as_str()
    };
    match Uuid::parse_str(s) {
        Ok(uuid) => uuid,
        Err(_) => Uuid::nil(),
    }
}

fn get_epoch<T>(timezone: &T) -> DateTime<T>
where
    T: TimeZone,
{
        timezone.ymd(1900, 1, 1).and_hms(0, 0, 0)
}

fn main() {
    println!("Hello, world! {}", Local::now());
    println!("utc       ={:?}", get_epoch(&Utc));
    println!("fixed+9   ={:?}", get_epoch(&FixedOffset::east(9 * 3600)));
    println!("fixed+9   ={:?}", get_epoch(&FixedOffset::east(0)));
    println!("local     ={:?}", get_epoch(&Local));
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
