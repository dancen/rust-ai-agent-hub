-- phpMyAdmin SQL Dump
-- version 5.2.0
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1:3306
-- Created on: Dec 07, 2025 at 09:30
-- Server version: 8.0.31
-- PHP version: 8.2.0

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `rust-ai-agent-hub`
--

-- --------------------------------------------------------

--
-- table `ai_agent`
--

DROP TABLE IF EXISTS `ai_agent`;
CREATE TABLE IF NOT EXISTS `ai_agent` (
  `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT,
  `token` char(64) NOT NULL,
  `user_id` bigint UNSIGNED NOT NULL,
  `name` varchar(100) NOT NULL,
  `data` longtext NOT NULL,
  `ttl` int UNSIGNED DEFAULT '86400',
  `created_at` datetime DEFAULT NULL,
  `updated_at` datetime DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `token` (`token`),
  KEY `user_id` (`user_id`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- --------------------------------------------------------

--
-- table `api_user`
--

DROP TABLE IF EXISTS `api_user`;
CREATE TABLE IF NOT EXISTS `api_user` (
  `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT,
  `username` varchar(100) NOT NULL,
  `api_key` char(64) NOT NULL,
  `rate_limit` int UNSIGNED DEFAULT '1000',
  `ttl` int UNSIGNED DEFAULT '3600',
  `created_at` datetime DEFAULT NULL,
  `updated_at` datetime DEFAULT NULL,
  `rate_count` int UNSIGNED DEFAULT NULL,
  `rate_reset_at` datetime DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `username` (`username`),
  UNIQUE KEY `api_key` (`api_key`)
) ENGINE=MyISAM AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dump table `api_user`
--

INSERT INTO `api_user` (`id`, `username`, `api_key`, `rate_limit`, `ttl`, `created_at`, `updated_at`, `rate_count`, `rate_reset_at`) VALUES
(1, 'agent_1234', 'q3f8sG7mV2J5K9d1Z0a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t1u2v3w4', 1000, 3600, '2025-12-05 20:55:45', '2025-12-05 20:55:45', 14, '2025-12-06 16:50:16');

-- --------------------------------------------------------

--
-- table `audit_log`
--

DROP TABLE IF EXISTS `audit_log`;
CREATE TABLE IF NOT EXISTS `audit_log` (
  `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT,
  `user_id` bigint UNSIGNED DEFAULT NULL,
  `action_type` varchar(50) NOT NULL,
  `target_table` varchar(50) NOT NULL,
  `target_id` bigint UNSIGNED DEFAULT NULL,
  `description` text,
  `ip_address` varchar(45) DEFAULT NULL,
  `created_at` datetime DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
