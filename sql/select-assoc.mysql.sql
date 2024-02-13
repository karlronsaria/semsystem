USE `mydb`;

SELECT
    `Name`
FROM
    `mydb`.`Item`
    LEFT JOIN
    `mydb`.`Item_has_Tag`
    ON `Id` = `Item_Id`
WHERE
    `Tag_Id` = (
        SELECT `Id`
        FROM `Tag`
        WHERE `Name` = 'finance'
    )
;

SELECT
    `Name`
FROM
    `mydb`.`Item`
    LEFT JOIN
    `mydb`.`Item_has_Tag`
    ON `Id` = `Item_Id`
WHERE
    `Tag_Id` = (
        SELECT `Id`
        FROM `Tag`
        WHERE `Name` = 'claim'
    )
;

SELECT
    `Name`
FROM
    `mydb`.`Item`
    LEFT JOIN
    `mydb`.`Item_has_Tag`
    ON `Id` = `Item_Id`
WHERE
    `Tag_Id` = (
        SELECT `Id`
        FROM `Tag`
        WHERE `Name` = 'auto'
    )
;

