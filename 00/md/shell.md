# Shell

通常我們會用 bash shell， bash shell 通常是 UNIX/Linux 與 Mac 共用的《終端機》指令系統，連 windows 的 Power Shell 也支援這些 UNIX指令。 

## 常用指令

* ls -- 列出此資料夾的檔案與子資料夾
* cd -- 切換資料夾
* mkdir -- 建立資料夾
* pwd -- 列出目前資料夾
* history -- 列出之前用過的指令歷史資料

使用範例：(此範例是在 Power Shell 中的操作結果)

```
PS D:\Dropbox\course\wd106b> ls


    目錄: D:\Dropbox\course\wd106b


Mode                LastWriteTime         Length Name
----                -------------         ------ ----
da----       2018/3/8  下午 05:51                example
da----       2018/3/2  上午 08:00                exercise
da----       2018/3/8  下午 05:49                more
da----       2018/3/2  上午 08:00                project
-a----       2018/3/9  下午 02:03            625 .gitignore
-a----       2018/3/9  下午 02:17            738 README.md


PS D:\Dropbox\course\wd106b> pwd

Path
----
D:\Dropbox\course\wd106b


PS D:\Dropbox\course\wd106b> cd example
PS D:\Dropbox\course\wd106b\example> mkdir test


    目錄: D:\Dropbox\course\wd106b\example


Mode                LastWriteTime         Length Name
----                -------------         ------ ----
d-----       2018/3/9  下午 02:53                test


PS D:\Dropbox\course\wd106b\example> ls


    目錄: D:\Dropbox\course\wd106b\example


Mode                LastWriteTime         Length Name
----                -------------         ------ ----
da----       2018/3/9  下午 02:03                01-html
da----       2018/3/8  下午 04:45                02-css
da----       2018/3/2  上午 07:26                03-bootstrap
da----       2018/3/1  上午 09:26                04-nodejs
da----     2017/12/28  下午 04:48                05-electronjs
da----       2018/3/2  上午 07:50                06-domjs
da----       2018/3/2  上午 07:36                08-jquery
da----       2018/3/8  下午 05:50                09-firebase
da----       2018/3/1  上午 08:32                10-vuejs
da----       2018/3/9  下午 02:53                test
-a----       2018/3/2  上午 08:00              0 README.md


PS D:\Dropbox\course\wd106b\example> rmdir test
PS D:\Dropbox\course\wd106b\example> ls


    目錄: D:\Dropbox\course\wd106b\example


Mode                LastWriteTime         Length Name
----                -------------         ------ ----
da----       2018/3/9  下午 02:03                01-html
da----       2018/3/8  下午 04:45                02-css
da----       2018/3/2  上午 07:26                03-bootstrap
da----       2018/3/1  上午 09:26                04-nodejs
da----     2017/12/28  下午 04:48                05-electronjs
da----       2018/3/2  上午 07:50                06-domjs
da----       2018/3/2  上午 07:36                08-jquery
da----       2018/3/8  下午 05:50                09-firebase
da----       2018/3/1  上午 08:32                10-vuejs
-a----       2018/3/2  上午 08:00              0 README.md

PS D:\Dropbox\course\wd106b\example> history

  Id CommandLine
  -- -----------
   1 cd .\wd106b\
   2 git add -A
   3 git commit -m "organize example"
   4 git push origin master
   5 git pull
   6 ls
   7 pwd
   8 cd example
   9 mkdir test
  10 ls
  11 rmdir test
  12 ls

```

## Windows 命令列與 Linux/MAC Shell 的基本操作

剛開始學習程式設計的同學，往往碰到的第一個障礙並不是寫程式，而是不知道如何下指令。

下什麼指令呢？

如果您用的是 windows 系統，那第一個障礙就是不知道如何下 DOS 指令了！

但是如果您用的是 Linux/MAC 系統，那第一個障礙就是不知道如何下 Shell 指令了！

DOS , 那不是已經被淘汰的東西嗎？

Oh ! DOS 系統雖然已經被 windows 系統取代二十幾年了，但是在 windows 裏的『命令提示字元』裏所用的指令，都還是當年的 DOS 指令，這也算是 DOS 所留下來的遺產吧！

在您初學程式之前，如果不會 DOS/Shell 指令，那沒有關係，我們只要先學會很簡單的幾個指令就行了，列表如下：

DOS 指令 | Shell 指令 | 說明 | 範例 | 範例解說
---------|-----------|------|------|---------
 cd | cd | change directory | cd /ccc/code/ | 切換到 /ccc/code/ 資料夾
dir | ls | directory | dir | 顯示目前資料夾中的檔案與子資料夾
 d: | 無磁碟機概念 | 切換磁碟機 | d: | 切換到 d 槽

## 進階參考

* [命令列的藝術](https://github.com/jlevy/the-art-of-command-line/blob/master/README-zh-Hant.md)


