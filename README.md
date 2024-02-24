# Dirchop
1. Provide the dirchop chop-command with a file or a directory.
2. Dirchop splits it into chunks of any size. (For example, a 100 megabyte directory can be split into 10 x 10MB files)
3. When all chunks of a given target are present, use the glue-command to recreate the original target.
## Usage
Use dirchop -h to get started.

Tar chunks use more space in total than the original because of metadata overhead, but often this is only kilobytes. If a target directory has a large number of small files, the metadata overhead may be larger.

When using dirchop glue, make sure that all chunks are present, and that chunks are not mixed between different targets. Either of these will corrupt the end result.

## Warning
Dirchop is not yet bug-free, use at your own discretion.
