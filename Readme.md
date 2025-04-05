# 1 Billion Rows challenge in RUST
I came across a this challenge while while watching [ThePrimeagen's video](https://www.youtube.com/watch?v=OO6l1DkYA0k) where he was reading an [article by Shraddha Agarwal](https://www.shraddhaag.dev/posts/1brc) about how she approached this challenge in Go. At the time I was learning Rust and I thought it would be a good idea to try and solve this challenge in Rust. I have not yet completed the challenge but I have made some progress. I will be updating this repository as I make progress.

## What is 1BRC?
Originally proposed by [Gunnar Morling](https://github.com/gunnarmorling/1brc) as a fun exploration of how far modern Java can be pushed for aggregating one billion rows from a text file. The challenge is to read a file with one billion rows, each row containing 
"<string: cityname>,<float: temperature>" and then calculate the average, min, and max temperature for each city. 


## Run details

Sr No. | Approach | Run 1 | Run 2 | Run 3 | Average | Delta From previous run | CommitID
--- | --- | --- | --- | --- | --- | --- | --- |
1 | Hashed Sorting | 29:48:981 | 33:14:439 | 40:43:485 | 34:35:635 | NA | [05f4d99](https://github.com/GuruprasadDalvi/1brc-rust/commit/05f4d99c518601a43b0361684724600291960091) |
2 | Buffred Reader | 24:41:324 | 24:37:976 | 29:57:156 | 25:45:147 | -8:50:488 | [bd64337](https://github.com/GuruprasadDalvi/1brc-rust/commit/bd64337ada632ce80459d357b6545a269571eeed) |
3 | Hashbrown |  22:35:810 | 22:33:435 | 22:34:487 | 22:34:433 | -3:11:286 |  |