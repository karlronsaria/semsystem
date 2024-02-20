@echo off

set "loginPath=sem-mydb"
set "db=mydb"
set "app=mysql --login-path=%loginPath%"

%app% -e "drop database if exists %db%;"
%app% -e "create database %db%;"

cat %~dp0./sql/function.mysql.sql | %app%

if "%~1" EQU "test" goto :test
goto :run

:test
cargo test -- --test-threads=1
goto :eof

:run
cargo %1

:: cat %~dp0./sql/select-assoc.mysql.sql | %app%

