# RV-Rust-Hypervisor

**A minimalist, research-oriented Type-1 Hypervisor for RISC-V 64, implemented in Rust 2024.**

---

## 🚀 Overview

**RV-Rust-Hypervisor** is a bare-metal **Type-1 hypervisor** that leverages the ratified **RISC-V Hypervisor (H) Extension**.

The project implements the fundamental building blocks of hardware-assisted virtualization, including:

* Memory isolation through **two-stage address translation**.
* Privilege-level switching between the **Host (HS-mode)** and **Guest (VS-mode)**.
* Guest trap interception and hypercall handling.
* Hardware-enforced guest memory isolation using **Sv39x4 G-Stage paging**.

The goal of this project is to provide a minimal and understandable foundation for experimenting with modern RISC-V virtualization.

---

## 🔬 Research Significance

Compared to legacy RISC-V virtualization implementations, this project explores several modern paradigms.

### 🦀 Memory Safety

The hypervisor is written in **`no_std` Rust 2024**, using modern raw-pointer semantics such as:

```rust
&raw mut SYMBOL
```

This provides safer interaction with:

* Linker-defined symbols.
* Memory-mapped I/O (MMIO).
* Bare-metal memory regions.

---

### ⚙️ Ratified RISC-V Hypervisor Extension

The implementation targets the official **RISC-V Hypervisor Extension v1.0**, rather than older draft specifications.

This provides a modern foundation for experimenting with hardware-assisted virtualization on RISC-V.

---

### 🧠 Two-Stage Address Translation

The hypervisor implements **Sv39x4 G-Stage paging** to translate:

```text
Guest Virtual Address (GVA)
        │
        ▼
Guest Physical Address (GPA)
        │
        ▼
Host Physical Address (HPA)
```

The G-Stage translation layer provides hardware-enforced memory isolation between the guest and the host.

---

### 🔧 Environment Configuration

The hypervisor configures `henvcfg` to control the guest execution environment and manage modern RISC-V virtualization behavior, including guest `ecall` handling.

This is relevant to modern RISC-V platform profiles such as **RVA22** and **RVA23**.

---

## ✨ Features

* **Type-1 Architecture**
  Runs directly on the system through OpenSBI as the primary supervisor-level virtualization layer.

* **RISC-V H-Extension**
  Uses hardware virtualization support provided by the ratified RISC-V Hypervisor Extension.

* **G-Stage Paging**
  Implements **Sv39x4** guest-stage address translation.

* **Large-Page Mapping**
  Identity-maps up to **4 GiB** of guest RAM and MMIO space using **1 GiB megapages**.

* **World Switching**
  Performs transitions from **HS-mode** to **VS-mode** using `sret` orchestration.

* **Trap-and-Emulate**
  Intercepts and handles guest environment calls and hypercalls.

* **Static Partitioning**
  Uses a minimal **Trusted Computing Base (TCB)** without dynamic scheduling overhead.

* **Bare-Metal Rust**
  Built using `#![no_std]` and `#![no_main]`.

---

## 🏗️ Architecture

```text
┌───────────────────────────────────────────────┐
│                  Guest VM                     │
│                                               │
│                  VS-Mode                      │
│                                               │
│     Guest Code │ UART Access │ Hypercalls     │
└───────────────────────┬───────────────────────┘
                        │
                        │ Trap / Virtualization
                        ▼
┌───────────────────────────────────────────────┐
│             RV-Rust-Hypervisor                │
│                                               │
│                  HS-Mode                      │
│                                               │
│   G-Stage Paging │ Trap Handler │ VM Control  │
└───────────────────────┬───────────────────────┘
                        │
                        │ SBI Interface
                        ▼
┌───────────────────────────────────────────────┐
│                   OpenSBI                     │
│                                               │
│                   M-Mode                      │
└───────────────────────┬───────────────────────┘
                        │
                        ▼
┌───────────────────────────────────────────────┐
│              RISC-V 64 Hardware               │
│          or QEMU `virt` Machine               │
└───────────────────────────────────────────────┘
```

### Privilege Hierarchy

```text
M-Mode   → OpenSBI
HS-Mode  → Hypervisor
VS-Mode  → Guest Operating Environment
```

---

## 📋 Prerequisites

Before building the project, ensure the following tools are installed:

* **Rust Toolchain:** Rust `1.82+` with Rust 2024 Edition support.
* **Compilation Target:** `riscv64gc-unknown-none-elf`.
* **Emulator:** QEMU `8.0+` with the RISC-V Hypervisor Extension enabled.
* **Firmware:** OpenSBI, typically provided automatically by QEMU's RISC-V `virt` machine.

---

## 📂 Project Structure

```text
.
├── .cargo/
│   └── config.toml        # Target configuration and custom linker arguments
│
├── src/
│   ├── linker.ld          # Memory layout, entry point, stack, and guest sections
│   └── main.rs            # Hypervisor core, UART, G-stage paging, and traps
│
├── Cargo.toml             # Rust package configuration
└── README.md
```

### Key Files

#### `.cargo/config.toml`

Configures:

* The `riscv64gc-unknown-none-elf` target.
* Custom linker arguments.
* Bare-metal build settings.

#### `src/linker.ld`

Defines the hypervisor memory layout, including:

* Entry point at `0x80200000`.
* Stack allocation.
* Hypervisor code and data sections.
* Dedicated guest code sections.

#### `src/main.rs`

Contains the core implementation:

* Hypervisor initialization.
* UART driver.
* CSR configuration.
* Sv39x4 G-Stage page-table construction.
* HS-mode → VS-mode transition.
* Trap handling.
* Guest hypercall interception.

---

## 🛠️ Building and Running

### 1. Install the RISC-V Target

```bash
rustup target add riscv64gc-unknown-none-elf
```

---

### 2. Compile the Hypervisor

```bash
cargo build
```

The compiled hypervisor binary will be generated at:

```text
target/riscv64gc-unknown-none-elf/debug/riscv-hypervisor
```

---

### 3. Run with QEMU

```bash
qemu-system-riscv64 \
    -machine virt \
    -cpu rv64,h=true \
    -nographic \
    -kernel target/riscv64gc-unknown-none-elf/debug/riscv-hypervisor
```

---

## 🖥️ Expected Output

Upon successful execution, the hypervisor will:

1. Initialize in **HS-mode**.
2. Configure the virtualization environment.
3. Build the G-Stage page tables.
4. Map the guest memory.
5. Transition into **VS-mode**.
6. Execute the guest code.
7. Intercept the guest hypercall.
8. Resume guest execution.

Example output:

```text
OpenSBI v1.5.1
...
[HYP] RISC-V Hypervisor Research Environment
[HYP] Entering Virtual VM at 0x00000000802007C0...
G
[HYP] Success! Guest Hypercall intercepted.
[HYP] Cause: 0x000000000000000A | Guest PC: 0x00000000802007CC
[HYP] Resuming Guest safely...
```

The `G` character demonstrates direct guest UART access, while the following messages confirm that the hypervisor successfully intercepted and handled the guest hypercall.

---

## 🔬 Implementation Details

| Component                | Implementation          |
| ------------------------ | ----------------------- |
| Architecture             | RISC-V 64               |
| Rust Edition             | Rust 2024               |
| Execution Model          | Type-1 Hypervisor       |
| Host Privilege Mode      | HS-mode                 |
| Guest Privilege Mode     | VS-mode                 |
| Virtualization Extension | RISC-V H-Extension v1.0 |
| G-Stage Translation      | Sv39x4                  |
| UART Base Address        | `0x10000000`            |
| Hypervisor Entry         | `0x80200000`            |
| Guest Code Section       | `.text.guest`           |
| Guest Alignment          | 16 bytes                |

---

## 🔄 Host-to-Guest Transition

The transition from the hypervisor to the guest is performed by configuring the required virtualization state before executing `sret`.

The hypervisor:

1. Configures G-Stage address translation.
2. Sets the guest execution environment.
3. Configures `hstatus.SPV`.
4. Configures `hstatus.SPVP`.
5. Sets the guest entry address.
6. Executes `sret`.

```text
HS-Mode
   │
   │ Configure hstatus
   │ Configure hgatp
   │ Set guest entry PC
   │
   ▼
  sret
   │
   ▼
VS-Mode
```

---

## 🪤 Trap-and-Emulate Flow

When the guest executes an operation requiring hypervisor intervention:

```text
Guest (VS-Mode)
      │
      │ ecall
      ▼
Hypervisor Trap
      │
      ├── Read trap cause
      ├── Inspect guest state
      ├── Handle hypercall
      ├── Advance guest PC
      │
      ▼
Resume Guest
```

This demonstrates one of the fundamental mechanisms used by modern hypervisors to control and virtualize guest execution.

---

## 🧠 Key Concepts Demonstrated

This project explores:

* RISC-V privilege architecture.
* RISC-V Hypervisor Extension.
* Bare-metal Rust development.
* `no_std` systems programming.
* Hardware-assisted virtualization.
* HS-mode and VS-mode execution.
* Two-stage address translation.
* Sv39x4 G-Stage paging.
* CSR manipulation.
* Trap handling.
* Hypercall interception.
* Memory-mapped I/O.
* Linker script design.
* Static partitioning.
* Trusted Computing Base minimization.

---

## 🎯 Project Goals

The primary goal of this project is to provide a minimal experimental platform for understanding modern RISC-V virtualization.

Potential future extensions include:

* Multiple guest virtual machines.
* Per-VM G-Stage page tables.
* Virtual interrupt injection.
* Virtual timer support.
* Guest device emulation.
* VM scheduling.
* Memory protection between multiple guests.
* VirtIO device support.
* Loading external guest binaries.

---

## 📜 License

This project is provided for **educational and research purposes**.

Feel free to use it as a foundation for your own experiments and research into **RISC-V virtualization**, **bare-metal Rust**, and **hypervisor development**.
