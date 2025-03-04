CREATE DATABASE sem;
USE sem;

-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema mydb
-- -----------------------------------------------------
DROP SCHEMA IF EXISTS `mydb` ;

-- -----------------------------------------------------
-- Schema mydb
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `mydb` DEFAULT CHARACTER SET utf8 ;
USE `mydb` ;

-- -----------------------------------------------------
-- Table `mydb`.`Item`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Item` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Item` (
  `Id` INT NOT NULL AUTO_INCREMENT,
  `Name` VARCHAR(45) NULL,
  `Arrival` DATE NULL,
  `Expiry` DATE NULL,
  `Content` LONGBLOB NULL,
  `Created` DATE NULL,
  PRIMARY KEY (`Id`),
  UNIQUE INDEX `Id_UNIQUE` (`Id` ASC) VISIBLE
)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mydb`.`Tag`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Tag` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Tag` (
  `Id` INT NOT NULL AUTO_INCREMENT,
  `Name` VARCHAR(45) NULL,
  `Created` DATE NULL,
  PRIMARY KEY (`Id`),
  UNIQUE INDEX `Id_UNIQUE` (`Id` ASC) VISIBLE
)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mydb`.`Date`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Date` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Date` (
  `Id` DATE NOT NULL,
  PRIMARY KEY (`Id`)
)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mydb`.`Item_has_Date`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Item_has_Date` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Item_has_Date` (
  `Item_Id` INT NOT NULL,
  `Date_Id` DATE NOT NULL,
  PRIMARY KEY (`Date_Id`, `Item_Id`),
  INDEX `fk_Item_has_Date_Date1_idx` (`Date_Id` ASC) VISIBLE,
  INDEX `fk_Item_has_Date_Item_idx` (`Item_Id` ASC) VISIBLE,
  CONSTRAINT `fk_Item_has_Date_Item`
    FOREIGN KEY (`Item_Id`)
    REFERENCES `mydb`.`Item` (`Id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_Item_has_Date_Date1`
    FOREIGN KEY (`Date_Id`)
    REFERENCES `mydb`.`Date` (`Id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION
)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mydb`.`Item_has_Tag`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Item_has_Tag` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Item_has_Tag` (
  `Item_Id` INT NOT NULL,
  `Tag_Id` INT NOT NULL,
  PRIMARY KEY (`Item_Id`, `Tag_Id`),
  INDEX `fk_Item_has_Tag_Tag1_idx` (`Tag_Id` ASC) VISIBLE,
  INDEX `fk_Item_has_Tag_Item1_idx` (`Item_Id` ASC) VISIBLE,
  CONSTRAINT `fk_Item_has_Tag_Item1`
    FOREIGN KEY (`Item_Id`)
    REFERENCES `mydb`.`Item` (`Id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_Item_has_Tag_Tag1`
    FOREIGN KEY (`Tag_Id`)
    REFERENCES `mydb`.`Tag` (`Id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION
)
ENGINE = InnoDB;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
