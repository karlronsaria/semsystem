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
    STR_TO_DATE('2023-10-01', '%Y-%m-%d'),
    NULL,
    STR_TO_DATE('2023-10-05', '%Y-%m-%d')
), (
    'Finance Statement 001.pdf',
    NULL,
    STR_TO_DATE('2023-11-01', '%Y-%m-%d'),
    NULL,
    STR_TO_DATE('2023-11-07', '%Y-%m-%d')
), (
    'Finance Statement 002.pdf',
    NULL,
    STR_TO_DATE('2023-12-01', '%Y-%m-%d'),
    NULL,
    STR_TO_DATE('2023-12-03', '%Y-%m-%d')
), (
    'Finance Statement 003.pdf',
    NULL,
    STR_TO_DATE('2024-01-01', '%Y-%m-%d'),
    NULL,
    STR_TO_DATE('2024-01-12', '%Y-%m-%d')
), (
    'Auto Claim - December 2022.pdf',
    NULL,
    STR_TO_DATE('2022-12-12', '%Y-%m-%d'),
    NULL,
    STR_TO_DATE('2022-12-12', '%Y-%m-%d')
), (
    'Auto Claim - January 2023.pdf',
    NULL,
    STR_TO_DATE('2023-01-13', '%Y-%m-%d'),
    NULL,
    STR_TO_DATE('2023-01-13', '%Y-%m-%d')
);

INSERT INTO `mydb`.`Tag` (
    Name,
    Created
) VALUES (
    'finance',
    STR_TO_DATE('2023-05-05', '%Y-%m-%d')
), (
    'auto',
    STR_TO_DATE('2023-12-12', '%Y-%m-%d')
), (
    'claim',
    STR_TO_DATE('2023-12-12', '%Y-%m-%d')
);

