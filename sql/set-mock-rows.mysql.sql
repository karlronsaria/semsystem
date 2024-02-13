USE `mydb`;

INSERT INTO `mydb`.`Item` (
    Name,
    Description,
    Arrival,
    Expiry,
    Created
) VALUES (
    'Finance Statement 000.pdf',
    NULL,
    STR_TO_DATE('2023_10_01', '%Y_%m_%d'),
    NULL,
    STR_TO_DATE('2023_10_05', '%Y_%m_%d')
), (
    'Finance Statement 001.pdf',
    NULL,
    STR_TO_DATE('2023_11_01', '%Y_%m_%d'),
    NULL,
    STR_TO_DATE('2023_11_07', '%Y_%m_%d')
), (
    'Finance Statement 002.pdf',
    NULL,
    STR_TO_DATE('2023_12_01', '%Y_%m_%d'),
    NULL,
    STR_TO_DATE('2023_12_03', '%Y_%m_%d')
), (
    'Finance Statement 003.pdf',
    NULL,
    STR_TO_DATE('2024_01_01', '%Y_%m_%d'),
    NULL,
    STR_TO_DATE('2024_01_12', '%Y_%m_%d')
), (
    'Auto Claim - December 2022.pdf',
    NULL,
    STR_TO_DATE('2022_12_12', '%Y_%m_%d'),
    NULL,
    STR_TO_DATE('2022_12_12', '%Y_%m_%d')
), (
    'Auto Claim - January 2023.pdf',
    NULL,
    STR_TO_DATE('2023_01_13', '%Y_%m_%d'),
    NULL,
    STR_TO_DATE('2023_01_13', '%Y_%m_%d')
);

INSERT INTO `mydb`.`Tag` (
    Name,
    Created
) VALUES (
    'finance',
    STR_TO_DATE('2023_05_05', '%Y_%m_%d')
), (
    'auto',
    STR_TO_DATE('2023_12_12', '%Y_%m_%d')
), (
    'claim',
    STR_TO_DATE('2023_12_12', '%Y_%m_%d')
);

