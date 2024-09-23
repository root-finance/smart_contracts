# Smart Contract Audit Report

## Project Overview

**Project Name:** Root Finance  
**Contract Name:** Lending Market\
**Audit Date:** 23 September 2024  
**Auditor(s):** Yevhenii Bezuhlyi     
**Programming Language:** Scrypto / Rust  
**Blockchain Platform:** Radix

## Table of Contents

1. [Project Overview](#project-overview)
2. [Introduction](#introduction)
3. [Scope of the Audit](#scope-of-the-audit)
4. [Audit Methodology](#audit-methodology)
5. [Highly Permissive Roles in the System](#highly-permissive-roles-in-the-system)
6. [Findings Summary](#findings-summary)
7. [Detailed Findings](#detailed-findings)
    - [Critical](#critical)
    - [Medium](#medium)
    - [Low](#low)
8. [General Information](#general-information)
    - [Code Quality and Best Practices](#code-quality-and-best-practices)

## Introduction

This audit report has been prepared for Root Finance. The purpose of this audit is to review the smart contract(s) for
Radix platform to identify potential security vulnerabilities, code issues, and other critical concerns.

## Scope of the Audit

Repository: `https://github.com/root-finance/hrc-smart_contracts`

Initial Commit: `4d8ed164b52d58bd688209a0e5ae038428620d98`\
Final Commit: `40ea51d0cb54235e2c245efbae103f77cb5001fc`

The following files/contracts were included in the audit:

| **File Path**                                         | **Lines of Code (LOC)** |
|-------------------------------------------------------|-------------------------|
| `lending_market\src\lending_market.rs`                | 287                     |
| `lending_market\src\lib.rs`                           | 3                       |
| `lending_market\src\resources.rs`                     | 130                     |
| `lending_market\src\modules\cdp_data.rs`              | 183                     |
| `lending_market\src\modules\cdp_health_checker.rs`    | 344                     |
| `lending_market\src\modules\interest_strategy.rs`     | 52                      |
| `lending_market\src\modules\liquidation_threshold.rs` | 106                     |
| `lending_market\src\modules\market_config.rs`         | 32                      |
| `lending_market\src\modules\mod.rs`                   | 9                       |
| `lending_market\src\modules\operation_status.rs`      | 109                     |
| `lending_market\src\modules\pool_config.rs`           | 167                     |
| `lending_market\src\modules\pool_state.rs`            | 118                     |
| `lending_market\src\modules\utils.rs`                 | 33                      |
| `single_resource_pool\src\lib.rs`                     | 105                     |
| `internal_price_feed\src\lib.rs`                      | 157                     |  

**Total LOC:** 1835

## Audit Methodology

The audit was conducted using the following methodology:

1. **Manual Code Review:** A line-by-line review of the smart contract code to identify potential vulnerabilities and
   logical errors.
2. **Test Cases and Simulations:** Execution of test cases to simulate real-world scenarios and assess the contract’s
   behavior under different conditions.
3. **Best Practices Check:** Assessment of adherence to best practices in smart contract development.

## Highly Permissive Roles in the System

### Admin Role

The most powerful role in the system. The admin can:

- **Create lending pools:** `create_lending_pool`
- **Update market configuration:** `update_market_config`
- **Update pool configuration:** `update_pool_config`
- **Update liquidation thresholds:** `update_liquidation_threshold`
- **Update interest strategies:** `update_interest_strategy`
- **Update price feeds:** `update_price_feed`
- **Update operating status with admin privileges:** `admin_update_operating_status`
- **Mint liquidator badges:** `mint_liquidator_badge`

The admin role can make significant changes to the core parameters and functionality of the lending market.

### Moderator Role

The moderator has some administrative capabilities, but more limited than the admin:

- **Update operating status:** `update_operating_status`

The moderator can enable/disable certain operations, but their changes can be overridden by an admin.

### Reserve Collector Role

This role has the ability to:

- **Collect reserve funds from all pools:** `collect_reserve`

The reserve collector can withdraw accumulated fees and interest from the lending pools.

### Liquidator Role

Liquidators have special privileges related to liquidating under-collateralized positions:

- **Start liquidation process:** `start_liquidation`
- **End liquidation process:** `end_liquidation`
- **Perform fast liquidation:** `fast_liquidation`

Liquidators play a crucial role in maintaining the overall health of the lending market by liquidating risky positions.

These roles have significant control over different aspects of the lending market's operation. The admin role, in
particular, has very broad powers to modify core parameters and functionality. Proper management and security around
these roles are critical for the safe operation of the system.

## Findings Summary

The following is a summary of the findings from the audit:

| **Severity** | **Issue**                                            | **Status** | **Notes**                                                                                                              |
|--------------|------------------------------------------------------|------------|------------------------------------------------------------------------------------------------------------------------|
| Critical     | Access Control Violation (C-01)                      | Fixed      |                                                                                                                        |
| Medium       | Lack of Liquidator Badge Revocation Mechanism (M-02) | Mitigated  | Since abilities of liquidators are limited by the contract logic and could not cause harm, the issue can be mitigated. |
| Medium       | Centralized Price Feed (M-03)                        | Mitigated  | Lack of thirdparty trusted oracles on the Radix chain makes can mitigate the issue until those will appear.            |
| Low          | Creation of Empty CDPs (L-01)                        | Fixed      |                                                                                                                        |
| Low          | Lack of Decimal Precision Handling (L-02)            | Mitigated  | Impact is extremely low.                                                                                               |

**Total Issues Found:** 5

## Detailed Findings

## Critical

### Access Control Violation (C-01)

- **Severity:** Critical
- **Impact:** High
- **Likelihood:** High
- **Type:** Access Control
- **Commit:** 4d8ed164
- **Status:** New
- **Target:** `./lending_market/src/lending_market.rs: fn take_batch_flashloan()`
- **Tests:** `./lending_merket/rests/blueprints/flashloan.rs: test_exploit_flashloan_by_burning_transient()`

#### Description:

A critical vulnerability has been identified in the flashloan mechanism of the lending market system. The current
implementation allows for unrestricted burning of `TransientResData` resources, which could be exploited to avoid
repaying flashloans.

**Steps to Reproduce:**

1. Set up a lending pool with sufficient liquidity.
2. Take out a flashloan.
3. Burn the `TransientResData` resource associated with the flashloan.
4. Keep the borrowed funds without repaying.

**Impact:**
This vulnerability could lead to significant financial losses for the lending pool and its depositors. It undermines the
entire flashloan mechanism and poses a severe risk to the stability and trustworthiness of the lending market.

#### Recommendation:

Modify the `TransientResData` resource in `resources.rs` to restrict burning. An example implementation is provided in
the audit commit.

## MEDIUM

### Lack of Liquidator Badge Revocation Mechanism (M-02)

- **Severity:** Low
- **Impact:** Medium
- **Likelihood:** Medium
- **Type:** Access Control
- **Commit:** 4d8ed164
- **Status:** Mitigated
- **Target:** `./lending_market/src/lending_market.rs: fn mint_liquidator_badge()`
- **Tests:** Not applicable

#### Description:

The `mint_liquidator_badge` function currently allows for the minting of liquidator badges but lacks a mechanism for
revoking them. While the function effectively creates new badges for users, the inability to revoke badges could present
potential risks in scenarios where a user's liquidator rights need to be revoked, such as in the case of misuse or after
a certain time period.

**Steps to Reproduce:**

1. Call the `mint_liquidator_badge` function to mint a new badge.
2. Observe that there is no corresponding function to revoke the badge once it has been minted.

**Impact:**
Without the ability to revoke liquidator badges,
there is a potential risk that users could retain liquidator privileges indefinitely,
even if they are no longer trusted or needed in that role.
This could lead to security concerns if the badges are misused.

#### Recommendation:

Implement a mechanism to revoke liquidator badges.
This would allow for better control over who holds liquidator privileges
and ensure that badges can be deactivated or reassigned as necessary.

### Centralized Price Feed (M-03)

- **Severity:** Medium
- **Impact:** High
- **Likelihood:** Low
- **Type:** Centralization Risk
- **Target:** Price feed mechanism
- **Commit:** 4d8ed164
- **Status:** Mitigated
- **Tests:** Not applicable

#### Description:

The current price feed mechanism relies on a single source of truth, which introduces a centralization risk.
If this single price feed is compromised or experiences downtime, it could affect the entire system's functionality.

**Impact:**
A compromised or malfunctioning price feed could lead to incorrect valuations,
potentially allowing users to borrow more than they should or triggering unnecessary liquidations.

#### Recommendation:

Implement a more robust, decentralized price feed system.
Consider using a combination of multiple price feeds and implementing a median or weighted average mechanism.

## LOW

### Creation of Empty Collateralized Debt Positions (CDPs) (L-01)

- **Severity:** Low
- **Impact:** Low
- **Likelihood:** Medium
- **Type:** Logic Flaw
- **Commit:** 4d8ed164
- **Status:** Fixed
- **Target:** `./lending_market/src/lending_market.rs: fn create_cdp()`
- **Tests:** Not applicable

#### Description:

A potential issue has been identified in the `create_cdp` function of the lending market system.
The current implementation allows the creation of a Collateralized Debt Position (CDP)
even when no collateral is provided (i.e., when the `deposits` vector is empty).
Although there is a note in the code indicating that the creation of empty CDPs should be forbidden,
this restriction is not enforced.

**Steps to Reproduce:**

1. Call the `create_cdp` function with an empty `deposits` vector.
2. Observe that a CDP is created without any collateral being added.

**Impact:**
Allowing the creation of empty CDPs could lead to unnecessary entries in the system,
resulting in potential clutter and making the management of CDPs more complex.
While this issue does not pose a direct security risk, it could cause confusion or inefficiencies in the system.

#### Recommendation:

Implement a check within the `create_cdp` function to prevent the creation
of a CDP if the `deposits` vector is empty.
If the vector is empty, the function should return an error or halt the process
to ensure that all CDPs have collateral.

## Lack of Decimal Precision Handling (L-02)

- **Severity:** Low
- **Impact:** Low
- **Likelihood:** Medium
- **Type:** Numerical Precision
- **Commit:** 4d8ed164
- **Status:** Mitigated
- **Target:** Throughout the codebase, especially in financial calculations
- **Tests:** Not applicable

### Description:

The project extensively uses the `Decimal` type for financial calculations,
but there's no consistent handling of decimal precision or rounding.
This could lead to small discrepancies in calculations,
which may accumulate over time or in high-volume scenarios.

### Impact:

Inconsistent rounding or precision issues could result in small but cumulative errors in interest calculations,
collateral valuations, or loan-to-value ratios.
Over time, this could lead to discrepancies between expected and actual balances.

### Recommendation:

Implement a standardized approach to decimal precision and rounding throughout the codebase.
Consider creating helper functions for financial calculations that enforce consistent precision and rounding rules.

## General Information

## Code Quality and Best Practices

### Code Readability and Documentation

#### Positives

1. Code is generally well-structured with clear module organization
2. Function names are descriptive and follow Rust naming conventions
3. Some inline comments explain complex logic, particularly in mathematical calculations

#### Areas for Improvement

1. Many functions lack documentation comments (///) explaining their purpose, parameters, and return values
2. Some complex calculations (e.g., in interest rate calculations) could benefit from more detailed explanations

### Adherence to Rust and Scrypto Best Practices

#### Positives

1. Proper use of Rust's type system, including enums for variant types
2. Consistent use of Result for error handling in many functions
3. Appropriate use of Scrypto-specific types like Decimal, PreciseDecimal, and ResourceAddress

#### Areas for Improvement

1. Some functions use `panic!` or `expect()` instead of returning Result for error handling
2. Inconsistent use of Decimal and PreciseDecimal types

### Code Duplication and Complexity

#### Positives

1. Use of macros like `save_cdp_macro!` and `emit_cdp_event!` to reduce code duplication
2. Separation of concerns into different modules (e.g., cdp_health_checker.rs, interest_strategy.rs)

#### Areas for Improvement

1. Some duplication in error checking and state updates across different functions
