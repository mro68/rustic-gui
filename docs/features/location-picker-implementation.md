# LocationPickerDialog Implementation Summary

**Date:** 2025-10-30  
**Mockup Reference:** `docs/mockups/rustic_location_picker.html`  
**Status:** ✅ Fully Implemented

## Overview

The LocationPickerDialog is a unified, tab-based dialog component for selecting repository storage locations across different backend types (Local, Network, Cloud). It replaces the simple file browser in AddRepositoryDialog with a comprehensive location selection interface.

## Implementation Details

### New Components

#### 1. LocationPickerDialog.svelte
**File:** `src/lib/components/dialogs/LocationPickerDialog.svelte`  
**Lines:** 543  
**Purpose:** Unified location picker with 4 tabs

**Features:**
- **📁 Local Tab:**
  - Integration with Tauri Dialog API (`@tauri-apps/plugin-dialog`)
  - Directory and file selection
  - New folder name input (for repository initialization)
  - Selected path display with validation
  - Info box showing final path preview

- **🌐 Network Tab:**
  - Protocol selection dropdown (SFTP, SMB, NFS, WebDAV)
  - Auto-port configuration based on protocol:
    - SFTP: Port 22
    - SMB/CIFS: Port 445
    - NFS: Port 2049
    - WebDAV: Port 443
  - Host and port inputs
  - Username input
  - Authentication method selection (Password/SSH Key)
  - Remote path input
  - Live URL preview in success info box

- **☁️ Cloud Tab:**
  - Grid of 7 cloud provider cards:
    1. Amazon S3 📦 - AWS Object Storage
    2. Backblaze B2 ☁️ - Affordable Cloud Storage
    3. Azure Blob 🔷 - Microsoft Cloud Storage
    4. Google Cloud 🌐 - GCS Object Storage
    5. Wasabi 💚 - Hot Cloud Storage
    6. MinIO 🪣 - Self-hosted S3-compatible
    7. Rclone 🔗 - 70+ Cloud Providers
  - Configuration form (shown after provider selection):
    - Endpoint/Region input
    - Bucket/Container name
    - Access Key/Client ID
    - Secret Key/Client Secret (password field)
  - Live cloud URL preview

- **🕐 Recent Tab:**
  - List of recently used locations
  - Icon-based type indicators (💾 Local, 🌐 Network, ☁️ Cloud)
  - Last used timestamp
  - Quick selection by clicking

**Props:**
```typescript
isOpen: boolean                      // Dialog visibility
mode: 'init' | 'open' = 'init'      // Repository mode
title: string                         // Dialog title
```

**Events:**
```typescript
select: { path: string; type: string; config?: any }  // Location selected
cancel: void                                           // Dialog cancelled
```

**Event Payloads:**

*Local:*
```typescript
{
  path: "/path/to/repo" or "/path/to/repo/new-folder",
  type: "local"
}
```

*Network:*
```typescript
{
  path: "sftp://user@host:port/path",
  type: "network",
  config: {
    protocol: "sftp",
    host: "backup.example.com",
    port: "22",
    username: "backupuser",
    auth: "password" | "key",
    remotePath: "/path/to/repo"
  }
}
```

*Cloud:*
```typescript
{
  path: "s3:s3.amazonaws.com/bucket",
  type: "cloud",
  config: {
    provider: "s3",
    endpoint: "s3.amazonaws.com",
    bucket: "my-backup",
    accessKey: "AKIAIOSFODNN7EXAMPLE",
    secretKey: "wJalrXUtnFEMI/...",
    region: "us-east-1"
  }
}
```

### Modified Components

#### 2. AddRepositoryDialog.svelte
**Changes:**
- Removed old `browseDirectory()` function using `@tauri-apps/plugin-dialog` directly
- Added `LocationPickerDialog` import
- Added state variables:
  - `showLocationPicker: boolean`
  - `locationConfig: any` (stores network/cloud configuration)
- Added functions:
  - `openLocationPicker()` - Opens the LocationPickerDialog
  - `handleLocationSelect(event)` - Handles location selection
  - `handleLocationCancel()` - Handles dialog cancellation
- Updated UI:
  - Replaced repository type selector cards with LocationPickerDialog integration
  - Path input is now readonly (filled by LocationPickerDialog)
  - Single "📁 Speicherort wählen" button
  - Shows repository type and path in help text
- LocationPickerDialog component added to template

#### 3. Modal.svelte
**Changes:**
- Added `size` prop:
  ```typescript
  size: 'small' | 'medium' | 'large' = 'medium'
  ```
- Updated template:
  - Added `modal-{size}` class to modal-dialog
- Added CSS classes:
  ```css
  .modal-small { max-width: 400px; }
  .modal-medium { max-width: 600px; }
  .modal-large { max-width: 900px; }
  ```

## Design Fidelity

### Color Palette (from mockup)
```css
--bg-primary: #1a1d2e
--bg-secondary: #22273a
--bg-tertiary: #2d3348
--border-color: #2d3348
--color-primary: #3b82f6
--color-success: #22c55e
--text-primary: #e4e4e7
--text-secondary: #71717a
```

### Layout Components

**Tabs:**
- Flex layout with 8px gap
- 12px padding, 20px horizontal
- Active tab: primary color bottom border (2px)
- Icons: 18px font-size
- Hover effect: color change

**Info Boxes:**
- Border-radius: 8px
- Primary: rgba(59, 130, 246, 0.1) background
- Success: rgba(34, 197, 94, 0.1) background
- 12px vertical, 16px horizontal padding

**Cloud Provider Grid:**
- Grid auto-fill with minmax(180px, 1fr)
- 12px gap
- Cards: 20px padding, 12px border-radius
- Hover: primary border color
- Selected: primary border + blue-tinted background

**Form Elements:**
- Label: 14px, font-weight 500
- Input: standard Input component
- Help text: 12px, secondary color

## Integration Flow

### User Journey

1. **User opens AddRepositoryDialog**
   - Clicks "Repository hinzufügen" in Repositories page
   - Or from Dashboard "+" button

2. **User clicks "📁 Speicherort wählen"**
   - LocationPickerDialog opens (size="large")
   - Default tab: Local

3. **User selects location:**

   **Option A: Local**
   - Clicks "📁 Verzeichnis wählen"
   - Native file picker opens
   - Selects directory
   - Optionally enters new folder name
   - Clicks "Speicherort wählen"

   **Option B: Network**
   - Switches to Network tab
   - Selects protocol (e.g., SFTP)
   - Enters host, username, remote path
   - Port auto-fills based on protocol
   - Reviews preview URL
   - Clicks "Speicherort wählen"

   **Option C: Cloud**
   - Switches to Cloud tab
   - Clicks provider card (e.g., Amazon S3)
   - Enters endpoint, bucket, credentials
   - Reviews preview URL
   - Clicks "Speicherort wählen"

   **Option D: Recent**
   - Switches to Recent tab
   - Clicks recent location
   - Automatically switches to appropriate tab and fills form

4. **LocationPickerDialog dispatches `select` event**
   - AddRepositoryDialog receives event
   - Updates `repositoryPath` and `repositoryType`
   - Stores `locationConfig` for network/cloud
   - Closes LocationPickerDialog

5. **User completes repository setup**
   - Enters repository name and password
   - Clicks "Repository hinzufügen"
   - AddRepositoryDialog submits with location data

## Backend Integration (To-Do)

The current implementation provides the UI layer. Backend integration requires:

### 1. Network Backend Support
**File:** `src-tauri/src/rustic/repository.rs`

```rust
pub async fn init_network_repository(
    protocol: &str,
    host: &str,
    port: u16,
    username: &str,
    remote_path: &str,
    password: &str,
) -> Result<Repository> {
    // Implement SFTP, SMB, NFS, WebDAV backends
}
```

### 2. Cloud Backend Support
**File:** `src-tauri/src/rustic/repository.rs`

```rust
pub async fn init_cloud_repository(
    provider: &str,
    endpoint: &str,
    bucket: &str,
    access_key: &str,
    secret_key: &str,
    password: &str,
) -> Result<Repository> {
    // Implement cloud provider backends
    // Use rustic_backend with OpenDAL
}
```

### 3. Recent Locations Persistence
**File:** `src-tauri/src/config.rs`

```rust
#[derive(Serialize, Deserialize)]
pub struct RecentLocation {
    pub path: String,
    pub type_: String,
    pub icon: String,
    pub last_used: DateTime<Utc>,
}

impl AppConfig {
    pub fn add_recent_location(&mut self, location: RecentLocation) {
        // Add to recent_locations list (max 10)
        // Save config
    }
}
```

### 4. Update API Wrapper
**File:** `src/lib/api/repositories.ts`

```typescript
export async function initRepository(
  path: string,
  password: string,
  type: 'local' | 'network' | 'cloud',
  config?: any
): Promise<RepositoryDto> {
  // Pass config to backend for network/cloud repos
}
```

## Testing Checklist

### Manual Testing

- [ ] **Local Tab:**
  - [ ] Directory selection works
  - [ ] File selection works (open mode)
  - [ ] New folder name is appended to path
  - [ ] Empty folder name uses selected path as-is
  - [ ] Preview shows correct path

- [ ] **Network Tab:**
  - [ ] Protocol selection changes port automatically
  - [ ] All protocols selectable (SFTP, SMB, NFS, WebDAV)
  - [ ] Preview URL updates live
  - [ ] Invalid host shows appropriately

- [ ] **Cloud Tab:**
  - [ ] All 7 providers selectable
  - [ ] Provider selection shows/hides config form
  - [ ] Credential fields accept input
  - [ ] Secret key field is password-masked
  - [ ] Preview URL updates live

- [ ] **Recent Tab:**
  - [ ] Recent locations display correctly
  - [ ] Clicking recent location selects it
  - [ ] Icon and type display correctly

- [ ] **General:**
  - [ ] Tab switching works smoothly
  - [ ] Cancel button closes dialog
  - [ ] Select button enabled/disabled based on selection
  - [ ] Modal backdrop closes dialog
  - [ ] ESC key closes dialog
  - [ ] Large modal size displays correctly
  - [ ] Responsive on smaller screens

### Integration Testing

- [ ] AddRepositoryDialog receives correct event payload
- [ ] Repository path updates correctly
- [ ] Repository type updates correctly
- [ ] Location config stored correctly
- [ ] Form validation works with LocationPicker data

## Known Limitations

1. **Recent Locations** - Currently hardcoded sample data. Needs persistence in backend config.

2. **Network/Cloud Backend** - UI is complete but backend implementation required for actual network/cloud repository support.

3. **File Browser** - Only uses Tauri's native file picker. No custom directory tree view (would require additional backend commands).

4. **Validation** - Basic validation in UI. Network URLs and cloud configs not validated beyond presence checks.

## Future Enhancements

1. **Connection Testing:**
   - Add "Test Connection" button for network/cloud
   - Verify credentials before selection
   - Show connection status

2. **Advanced Network Options:**
   - SSH key file selection for SFTP
   - Domain/Workgroup for SMB
   - Mount options for NFS

3. **Cloud Region Selection:**
   - Dropdown for common regions per provider
   - Auto-complete for endpoint URLs

4. **Recent Locations Management:**
   - Clear recent history
   - Pin favorite locations
   - Delete individual recent items

5. **Accessibility:**
   - Keyboard navigation between tabs
   - ARIA labels for provider cards
   - Screen reader announcements for selection

## Files Modified

```
src/lib/components/dialogs/LocationPickerDialog.svelte  (NEW - 543 lines)
src/lib/components/dialogs/AddRepositoryDialog.svelte   (MODIFIED)
src/lib/components/shared/Modal.svelte                  (MODIFIED)
TODO.md                                                  (UPDATED)
```

## Git Commits

1. `feat: Add LocationPickerDialog component with multi-tab location selection`
2. `docs: Update TODO.md with LocationPickerDialog completion status`

## Related Issues

- Addresses mockup integration requirement: `docs/mockups/rustic_location_picker.html`
- Part of TODO.md Phase 2: Dialog-Workflow: Repository
- Enhances AddRepositoryDialog with unified location selection

## Screenshots

*Screenshots would be taken from running application - requires Tauri runtime which is not available in this environment.*

For screenshots, run:
```bash
npm run tauri:dev
```

Then capture:
1. LocationPickerDialog - Local Tab
2. LocationPickerDialog - Network Tab (SFTP)
3. LocationPickerDialog - Cloud Tab (S3 selected)
4. LocationPickerDialog - Recent Tab
5. AddRepositoryDialog with LocationPicker integration
