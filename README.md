# 👊 `punch`

A simple time-tracking CLI tool written in Rust. Punch in, punch out, and pretty-print records to the terminal in different time granularity.

## How-To

`in`, `out`, `show` - that's all you need:

- **`punch in [<note>]`**: Start tracking time and pass an optional note.
- **`punch out [<note>]`**: Stop tracking time and pass an optional note.
- **`punch show [day|week|month|year] [--precise] [--round ROUNDING]`**: Print tracked times and notes grouped by the specified time interval to console. (default: `week`).
    - `--precise/-p`: prints timestamps in RFC 3339 format.
    - `--round/-r ROUNDING`: rounds durations according to the specified options in `ROUNDING`. Examples: `up,15min`, `down,1h`, `nearest,1day`.

Each `punch in` must be followed by a `punch out`. You can't `punch in` if you haven't `punch out`d the previous record. The tool performs some validation on each punch and reports invalid state.

If you punched by mistake, you can manually edit the punch card at any time in `~/.punch/main.csv`. Each record is CSV-encoded by `index,start,[end],[note]`, where `[]` denotes optional fields.

## Example

Running `punch show day` based on some test data:

![terminal output](./screenshot.png)

## Future improvements
- Support multiple punch cards and allow switching between them. E.g. with `punch list` and `punch switch`.
- Global stats
- OS-integration would be sweet: Act on shutdown/start/sleep/opening terminal/Slack.
- Collect more use-cases. Minute granularity viable?
- Print table on `punch show`?
- Use more consistent practises:
    * error-handling: when is `expect` fine, when `chain_err`, when `map_err`
    * display structs: fmt::Display does not accept params, so either Formatter module or `.display_with()` can be used.
