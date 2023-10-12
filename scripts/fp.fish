function fp
    if test (count $argv) -lt 1
        echo "用法: fp <subcommand>"
        echo "模糊拼音搜索工具"
        return 1
    end
    if test "$argv[1]" = "cd"
        set dir (skim-pinyin -d)
        if test -n "$dir"
            builtin cd $dir
        end
    else
        set files (skim-pinyin -m)
        if test -n "$files"
            $argv[1..-1] $files
        end
    end
end
