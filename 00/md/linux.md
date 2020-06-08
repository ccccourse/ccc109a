# linux

我主要使用的是 ubuntu linux

## 群組

* [Add a User to a Group (or Second Group) on Linux](https://www.howtogeek.com/50787/add-a-user-to-a-group-or-second-group-on-linux/)

## 改變資料夾權限

> chmod -R 777 目錄

## 新增使用者

ubuntu 的 useradd 不好用，登入之後會沒有  /home/user 的家目錄，因為：

useradd is a low level utility for adding users. On Debian, administrators should usually use adduser(8) instead.

所以應改用 adduser

```
root@localhost:~# sudo adduser ccc3
Adding user `ccc3' ...
Adding new group `ccc3' (1002) ...
Adding new user `ccc3' (1002) with group `ccc3' ...
Creating home directory `/home/ccc3' ...
Copying files from `/etc/skel' ...
Enter new UNIX password:
Retype new UNIX password:
passwd: password updated successfully
Changing the user information for ccc3
Enter the new value, or press ENTER for the default
        Full Name []: Chung-Chen Chen
        Room Number []:
        Work Phone []:
        Home Phone []:
        Other []:
Is the information correct? [Y/n] y
```

刪除使用者

```
$ deluser <user>
```

## List Users

* [How to List Users in Linux](https://linuxize.com/post/how-to-list-users-in-linux/)
