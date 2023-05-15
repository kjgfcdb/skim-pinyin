function fp
    if test (count $argv) -lt 1
        echo "Invalid number of arguments"
        echo "Usage fp <subcommand>"
        return 1
    end
    if test "$argv[1]" = "vim"
        vim (skim-pinyin -m)
    else if test "$argv[1]" = "cd"
        builtin cd (skim-pinyin -d)
    else if test "$argv[1]" = "rm"
        if test (count $argv) -eq 2 -a "$argv[2]" = "-rf"
            rm -rf (skim-pinyin -m)
        else
            rm (skim-pinyin -m)
        end
    else
        echo "Unsupported subcommand! Please edit ~/.config/fish/functions/fp.fish to add your own subcommand."
    end
end
