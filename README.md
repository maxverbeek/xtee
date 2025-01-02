# XTee: eXecute Tee

A utility to filter input via a regular expression, and pass this input into a program.

Intended use is to use this in combination with `git push` as follows:

```sh
git push -o merge_request.create 2>&1 | xtee -p "https://\\S+" -e wl-copy >&2
```

Which should pass the stderr of git push to the stdin of xtee. xtee will
filter out any URLs via the https regex, and forward it to wl-copy which copies
it to the clipboard. The output of git push usually includes a merge request
URL if you create one via the push flags. I want to grab these URLs as this
output is generated in the background. Since I use nvim this text is hard to
copy, since it appears in a notification that is removed when I click on it.

Since xtee will "tee" the output to stdout as well, it should still show up.
Since I stole the stderr, I want to dump the output back to stderr, not sure if
that makes a difference for how nvim displays these messages.

An important requirement is that text should be "live" fed to stdout without
much buffer delay, so that I can see my push progress. Any escape sequences
should be unmodified so that git push will correctly render progress bars.
