USE `mydb`;

INSERT INTO `mydb`.`Item_has_Tag`
    (`Item_Id`, `Tag_Id`)
SELECT
    a.`Id`, b.`Id`
FROM
    `mydb`.`Item` as a
    JOIN
    `mydb`.`Tag` as b
WHERE
    a.`Name` LIKE 'Finance%'
    AND
    b.`Name` = 'finance'
ON DUPLICATE KEY UPDATE
    `Item_Id` = a.`Id`,
    `Tag_Id` = b.`Id`
;

INSERT INTO `mydb`.`Item_has_Tag`
    (`Item_Id`, `Tag_Id`)
SELECT
    a.`Id`, b.`Id`
FROM
    `mydb`.`Item` as a
    JOIN
    `mydb`.`Tag` as b
WHERE
    a.`Name` LIKE 'Auto%'
    AND
    b.`Name` = 'auto'
ON DUPLICATE KEY UPDATE
    `Item_Id` = a.`Id`,
    `Tag_Id` = b.`Id`
;

INSERT INTO `mydb`.`Item_has_Tag`
    (`Item_Id`, `Tag_Id`)
SELECT
    a.`Id`, b.`Id`
FROM
    `mydb`.`Item` as a
    JOIN
    `mydb`.`Tag` as b
WHERE
    a.`Name` LIKE 'Claim%'
    AND
    b.`Name` = 'claim'
ON DUPLICATE KEY UPDATE
    `Item_Id` = a.`Id`,
    `Tag_Id` = b.`Id`
;

