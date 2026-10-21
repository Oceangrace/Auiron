use std::path::PathBuf;

fn main() -> Result<(), String> {
    let mut args = std::env::args_os().skip(1);
    let first = args
        .next()
        .ok_or("Usage: aurion-traffic <file.pcap> <server-port> [events.jsonl], --probe, --interfaces <file.pcap> <server-port> <profiles>, --record <seconds> <new-directory>, or --record-marked <seconds> <new-directory>")?;

    if first == "--probe" {
        if args.next().is_some() {
            return Err("Unexpected probe argument".into());
        }

        println!("{}", aurion_lib::probe_capture()?);

        return Ok(());
    }

    if first == "--journal" {
        let recording = args
            .next()
            .map(PathBuf::from)
            .ok_or("Recording path is missing")?;
        let port = args
            .next()
            .and_then(|value| value.to_str().and_then(|value| value.parse::<u16>().ok()))
            .ok_or("Server port is missing")?;
        let output = args
            .next()
            .map(PathBuf::from)
            .ok_or("Journal output path is missing")?;

        if args.next().is_some() {
            return Err("Unexpected journal inspection argument".into());
        }

        println!(
            "{}",
            aurion_lib::inspect_journal(&recording, port, &output)?
        );

        return Ok(());
    }

    if first == "--interfaces" {
        let recording = args
            .next()
            .map(PathBuf::from)
            .ok_or("Recording path is missing")?;
        let port = args
            .next()
            .and_then(|value| value.to_str().and_then(|value| value.parse::<u16>().ok()))
            .ok_or("Server port is missing")?;
        let workspace = args
            .next()
            .map(PathBuf::from)
            .ok_or("Workspace path is missing")?;

        if args.next().is_some() {
            return Err("Unexpected interface inspection argument".into());
        }

        println!(
            "{}",
            aurion_lib::inspect_interfaces(&recording, port, &workspace)?
        );

        return Ok(());
    }

    if first == "--record" || first == "--record-marked" {
        let seconds = args
            .next()
            .and_then(|value| value.to_str().and_then(|value| value.parse::<u64>().ok()))
            .ok_or("Recording duration is missing")?;
        let directory = args
            .next()
            .map(PathBuf::from)
            .ok_or("Recording directory is missing")?;

        if args.next().is_some() {
            return Err("Unexpected recording argument".into());
        }

        let report = if first == "--record-marked" {
            aurion_lib::record_marked_traffic(&directory, seconds)?
        } else {
            aurion_lib::record_traffic(&directory, seconds)?
        };

        println!("{report}");

        return Ok(());
    }

    let path = PathBuf::from(first);
    let port = args
        .next()
        .and_then(|value| value.to_str().and_then(|value| value.parse::<u16>().ok()))
        .ok_or("Expected a server port between 1 and 65535")?;
    let events_path = args.next().map(PathBuf::from);

    if args.next().is_some() {
        return Err("Unexpected extra argument".into());
    }

    println!(
        "{}",
        aurion_lib::inspect_traffic(&path, port, events_path.as_deref())?
    );

    Ok(())
}
