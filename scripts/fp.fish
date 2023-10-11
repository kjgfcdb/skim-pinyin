function fp
    if test (count $argv) -lt 1
        echo "用法: fp <subcommand>"
        echo "模糊拼音搜索工具"
        return 1
    end
    if test "$argv[1]" = "cd"
        builtin cd (skim-pinyin -d)
    else
        $argv[1..-1] (skim-pinyin -m)
    end
end
