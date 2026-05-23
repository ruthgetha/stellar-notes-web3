# Stellar Notes DApp (Multi-User Collaboration + Media Support)

> Decentralized Collaborative Note-Taking System with Image & Video Drop Support on Stellar Soroban

---

# Project Description

**Stellar Notes DApp** is a production-ready decentralized collaboration platform built on the **Stellar Blockchain** using the **Soroban Smart Contract SDK**.

The system enables users to create, manage, and collaborate on shared notes in a fully decentralized environment. Each note is owned by a user but can include multiple collaborators with role-based permissions.

In addition, the platform supports **media attachments (image/video)** through decentralized storage references (CID-based systems such as IPFS).

This project focuses on:
- multi-user collaboration
- role-based access control
- decentralized ownership
- multimedia note support
- blockchain-based security and transparency

---

# Project Vision

Our vision is to redefine digital collaboration by building a trustless and decentralized workspace where users fully own their data.

We aim to:

- **Enable Decentralized Collaboration**
  Allow multiple users to work on shared notes without centralized servers.

- **Ensure Data Ownership**
  Every note is owned and controlled by its creator.

- **Provide Role-Based Access Control**
  Secure collaboration with Owner, Editor, and Viewer roles.

- **Support Multimedia Workflows**
  Enable image and video attachments in decentralized notes.

- **Guarantee Transparency & Security**
  All operations are verified on-chain.

We envision a future where collaboration tools are:
- user-owned
- permission-controlled
- censorship-resistant
- globally accessible

---

# Key Features

---

## 1. Multi-User Collaboration System

Each note supports multiple collaborators with different roles.

### Roles

### Owner
- full control of note
- manage collaborators
- delete note

### Editor
- can modify note content
- can upload media

### Viewer
- read-only access
- view notes and media

---

## 2. Decentralized Note Creation

Users can create blockchain-based notes with:

- title
- content
- ownership binding
- auto-generated ID

### Features

- immutable storage
- secure ownership validation
- persistent blockchain records
- automatic collaboration initialization

---

## 3. Drag & Drop Image & Video Support

The platform supports multimedia attachments using decentralized storage references.

### Supported Media Types

#### Images
- PNG
- JPG
- JPEG
- WEBP

#### Videos
- MP4
- MOV
- WEBM

---

### Media Upload Flow

```text
User Uploads Image / Video
        ↓
Frontend Upload System
        ↓
IPFS / Decentralized Storage
        ↓
CID Hash Generated
        ↓
CID Stored in Smart Contract