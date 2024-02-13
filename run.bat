@echo off

set "loginPath=sem-mydb"
set "db=mydb"
set "app=mysql --login-path=%loginPath%"

%app% -e "drop database if exists %db%;"
%app% -e "create database %db%;"

cargo %1

cat %~dp0./sql/select-assoc.mysql.sql | %app%

