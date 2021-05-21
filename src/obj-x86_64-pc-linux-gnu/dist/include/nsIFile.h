/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIFile.idl
 */

#ifndef __gen_nsIFile_h__
#define __gen_nsIFile_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIDirectoryEnumerator_h__
#include "nsIDirectoryEnumerator.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
struct PRFileDesc;
struct PRLibrary;
#include <stdio.h>
#include "mozilla/Path.h"
#include "nsCOMPtr.h"
#include "nsStringFwd.h"
namespace mozilla {
using PathString = nsTString<filesystem::Path::value_type>;
using PathSubstring = nsTSubstring<filesystem::Path::value_type>;
} // namespace mozilla

/* starting interface:    nsIFile */
#define NS_IFILE_IID_STR "2fa6884a-ae65-412a-9d4c-ce6e34544ba1"

#define NS_IFILE_IID \
  {0x2fa6884a, 0xae65, 0x412a, \
    { 0x9d, 0x4c, 0xce, 0x6e, 0x34, 0x54, 0x4b, 0xa1 }}

class nsIFile : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFile;

  enum {
    NORMAL_FILE_TYPE = 0U,
    DIRECTORY_TYPE = 1U
  };

  /* void append (in AString node); */
  NS_IMETHOD Append(const nsAString& node) = 0;

  /* [noscript] void appendNative (in ACString node); */
  NS_IMETHOD AppendNative(const nsACString& node) = 0;

  /* void normalize (); */
  NS_IMETHOD Normalize(void) = 0;

  /* [must_use] void create (in unsigned long type, in unsigned long permissions); */
  [[nodiscard]] NS_IMETHOD Create(uint32_t type, uint32_t permissions) = 0;

  /* attribute AString leafName; */
  NS_IMETHOD GetLeafName(nsAString& aLeafName) = 0;
  NS_IMETHOD SetLeafName(const nsAString& aLeafName) = 0;

  /* [noscript] attribute ACString nativeLeafName; */
  NS_IMETHOD GetNativeLeafName(nsACString& aNativeLeafName) = 0;
  NS_IMETHOD SetNativeLeafName(const nsACString& aNativeLeafName) = 0;

  /* void copyTo (in nsIFile newParentDir, in AString newName); */
  NS_IMETHOD CopyTo(nsIFile *newParentDir, const nsAString& newName) = 0;

  /* [noscript] void CopyToNative (in nsIFile newParentDir, in ACString newName); */
  NS_IMETHOD CopyToNative(nsIFile *newParentDir, const nsACString& newName) = 0;

  /* void copyToFollowingLinks (in nsIFile newParentDir, in AString newName); */
  NS_IMETHOD CopyToFollowingLinks(nsIFile *newParentDir, const nsAString& newName) = 0;

  /* [noscript] void copyToFollowingLinksNative (in nsIFile newParentDir, in ACString newName); */
  NS_IMETHOD CopyToFollowingLinksNative(nsIFile *newParentDir, const nsACString& newName) = 0;

  /* void moveTo (in nsIFile newParentDir, in AString newName); */
  NS_IMETHOD MoveTo(nsIFile *newParentDir, const nsAString& newName) = 0;

  /* [noscript] void moveToNative (in nsIFile newParentDir, in ACString newName); */
  NS_IMETHOD MoveToNative(nsIFile *newParentDir, const nsACString& newName) = 0;

  /* void moveToFollowingLinks (in nsIFile newParentDir, in AString newName); */
  NS_IMETHOD MoveToFollowingLinks(nsIFile *newParentDir, const nsAString& newName) = 0;

  /* [noscript] void moveToFollowingLinksNative (in nsIFile newParentDir, in ACString newName); */
  NS_IMETHOD MoveToFollowingLinksNative(nsIFile *newParentDir, const nsACString& newName) = 0;

  /* void renameTo (in nsIFile newParentDir, in AString newName); */
  NS_IMETHOD RenameTo(nsIFile *newParentDir, const nsAString& newName) = 0;

  /* [noscript] void renameToNative (in nsIFile newParentDir, in ACString newName); */
  NS_IMETHOD RenameToNative(nsIFile *newParentDir, const nsACString& newName) = 0;

  /* void remove (in boolean recursive); */
  NS_IMETHOD Remove(bool recursive) = 0;

  /* attribute unsigned long permissions; */
  NS_IMETHOD GetPermissions(uint32_t *aPermissions) = 0;
  NS_IMETHOD SetPermissions(uint32_t aPermissions) = 0;

  /* attribute unsigned long permissionsOfLink; */
  NS_IMETHOD GetPermissionsOfLink(uint32_t *aPermissionsOfLink) = 0;
  NS_IMETHOD SetPermissionsOfLink(uint32_t aPermissionsOfLink) = 0;

  /* attribute PRTime lastModifiedTime; */
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) = 0;
  NS_IMETHOD SetLastModifiedTime(PRTime aLastModifiedTime) = 0;

  /* attribute PRTime lastModifiedTimeOfLink; */
  NS_IMETHOD GetLastModifiedTimeOfLink(PRTime *aLastModifiedTimeOfLink) = 0;
  NS_IMETHOD SetLastModifiedTimeOfLink(PRTime aLastModifiedTimeOfLink) = 0;

  /* readonly attribute PRTime creationTime; */
  NS_IMETHOD GetCreationTime(PRTime *aCreationTime) = 0;

  /* readonly attribute PRTime creationTimeOfLink; */
  NS_IMETHOD GetCreationTimeOfLink(PRTime *aCreationTimeOfLink) = 0;

  /* attribute int64_t fileSize; */
  NS_IMETHOD GetFileSize(int64_t *aFileSize) = 0;
  NS_IMETHOD SetFileSize(int64_t aFileSize) = 0;

  /* readonly attribute int64_t fileSizeOfLink; */
  NS_IMETHOD GetFileSizeOfLink(int64_t *aFileSizeOfLink) = 0;

  /* readonly attribute AString target; */
  NS_IMETHOD GetTarget(nsAString& aTarget) = 0;

  /* [noscript] readonly attribute ACString nativeTarget; */
  NS_IMETHOD GetNativeTarget(nsACString& aNativeTarget) = 0;

  /* readonly attribute AString path; */
  NS_IMETHOD GetPath(nsAString& aPath) = 0;

  /* [must_use,nostdcall,notxpcom] PathString nativePath (); */
  [[nodiscard]] virtual mozilla::PathString NativePath(void) = 0;

 #ifndef XP_WIN
    nsresult GetNativePath(nsACString& aPath);
#endif
    /*
     * Returns a human-readable path string.
     */
    nsCString HumanReadablePath();
  /* boolean exists (); */
  NS_IMETHOD Exists(bool *_retval) = 0;

  /* boolean isWritable (); */
  NS_IMETHOD IsWritable(bool *_retval) = 0;

  /* boolean isReadable (); */
  NS_IMETHOD IsReadable(bool *_retval) = 0;

  /* boolean isExecutable (); */
  NS_IMETHOD IsExecutable(bool *_retval) = 0;

  /* boolean isHidden (); */
  NS_IMETHOD IsHidden(bool *_retval) = 0;

  /* boolean isDirectory (); */
  NS_IMETHOD IsDirectory(bool *_retval) = 0;

  /* boolean isFile (); */
  NS_IMETHOD IsFile(bool *_retval) = 0;

  /* boolean isSymlink (); */
  NS_IMETHOD IsSymlink(bool *_retval) = 0;

  /* boolean isSpecial (); */
  NS_IMETHOD IsSpecial(bool *_retval) = 0;

  /* [must_use] void createUnique (in unsigned long type, in unsigned long permissions); */
  [[nodiscard]] NS_IMETHOD CreateUnique(uint32_t type, uint32_t permissions) = 0;

  /* nsIFile clone (); */
  NS_IMETHOD Clone(nsIFile **_retval) = 0;

  /* boolean equals (in nsIFile inFile); */
  NS_IMETHOD Equals(nsIFile *inFile, bool *_retval) = 0;

  /* boolean contains (in nsIFile inFile); */
  NS_IMETHOD Contains(nsIFile *inFile, bool *_retval) = 0;

  /* readonly attribute nsIFile parent; */
  NS_IMETHOD GetParent(nsIFile **aParent) = 0;

  /* [binaryname(DirectoryEntriesImpl)] readonly attribute nsIDirectoryEnumerator directoryEntries; */
  NS_IMETHOD GetDirectoryEntriesImpl(nsIDirectoryEnumerator **aDirectoryEntries) = 0;

     nsresult GetDirectoryEntries(nsIDirectoryEnumerator** aOut)
    {
      return GetDirectoryEntriesImpl(aOut);
    };
      /* void initWithPath (in AString filePath); */
  NS_IMETHOD InitWithPath(const nsAString& filePath) = 0;

  /* [noscript] void initWithNativePath (in ACString filePath); */
  NS_IMETHOD InitWithNativePath(const nsACString& filePath) = 0;

  /* void initWithFile (in nsIFile aFile); */
  NS_IMETHOD InitWithFile(nsIFile *aFile) = 0;

  enum {
    OS_READAHEAD = 1073741824U,
    DELETE_ON_CLOSE = 2147483648U
  };

  /* [must_use,noscript] PRFileDescStar openNSPRFileDesc (in long flags, in long mode); */
  [[nodiscard]] NS_IMETHOD OpenNSPRFileDesc(int32_t flags, int32_t mode, PRFileDesc * * _retval) = 0;

  /* [must_use,noscript] FILE openANSIFileDesc (in string mode); */
  [[nodiscard]] NS_IMETHOD OpenANSIFileDesc(const char * mode, FILE * * _retval) = 0;

  /* [must_use,noscript] PRLibraryStar load (); */
  [[nodiscard]] NS_IMETHOD Load(PRLibrary * * _retval) = 0;

  /* [must_use] readonly attribute int64_t diskSpaceAvailable; */
  [[nodiscard]] NS_IMETHOD GetDiskSpaceAvailable(int64_t *aDiskSpaceAvailable) = 0;

  /* void appendRelativePath (in AString relativeFilePath); */
  NS_IMETHOD AppendRelativePath(const nsAString& relativeFilePath) = 0;

  /* [noscript] void appendRelativeNativePath (in ACString relativeFilePath); */
  NS_IMETHOD AppendRelativeNativePath(const nsACString& relativeFilePath) = 0;

  /* [must_use] attribute ACString persistentDescriptor; */
  [[nodiscard]] NS_IMETHOD GetPersistentDescriptor(nsACString& aPersistentDescriptor) = 0;
  [[nodiscard]] NS_IMETHOD SetPersistentDescriptor(const nsACString& aPersistentDescriptor) = 0;

  /* [must_use] void reveal (); */
  [[nodiscard]] NS_IMETHOD Reveal(void) = 0;

  /* [must_use] void launch (); */
  [[nodiscard]] NS_IMETHOD Launch(void) = 0;

  /* [must_use] ACString getRelativeDescriptor (in nsIFile fromFile); */
  [[nodiscard]] NS_IMETHOD GetRelativeDescriptor(nsIFile *fromFile, nsACString& _retval) = 0;

  /* [must_use] void setRelativeDescriptor (in nsIFile fromFile, in ACString relativeDesc); */
  [[nodiscard]] NS_IMETHOD SetRelativeDescriptor(nsIFile *fromFile, const nsACString& relativeDesc) = 0;

  /* [must_use] AUTF8String getRelativePath (in nsIFile fromFile); */
  [[nodiscard]] NS_IMETHOD GetRelativePath(nsIFile *fromFile, nsACString& _retval) = 0;

  /* [must_use] void setRelativePath (in nsIFile fromFile, in AUTF8String relativeDesc); */
  [[nodiscard]] NS_IMETHOD SetRelativePath(nsIFile *fromFile, const nsACString& relativeDesc) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFile, NS_IFILE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILE \
  NS_IMETHOD Append(const nsAString& node) override; \
  NS_IMETHOD AppendNative(const nsACString& node) override; \
  NS_IMETHOD Normalize(void) override; \
  [[nodiscard]] NS_IMETHOD Create(uint32_t type, uint32_t permissions) override; \
  NS_IMETHOD GetLeafName(nsAString& aLeafName) override; \
  NS_IMETHOD SetLeafName(const nsAString& aLeafName) override; \
  NS_IMETHOD GetNativeLeafName(nsACString& aNativeLeafName) override; \
  NS_IMETHOD SetNativeLeafName(const nsACString& aNativeLeafName) override; \
  NS_IMETHOD CopyTo(nsIFile *newParentDir, const nsAString& newName) override; \
  NS_IMETHOD CopyToNative(nsIFile *newParentDir, const nsACString& newName) override; \
  NS_IMETHOD CopyToFollowingLinks(nsIFile *newParentDir, const nsAString& newName) override; \
  NS_IMETHOD CopyToFollowingLinksNative(nsIFile *newParentDir, const nsACString& newName) override; \
  NS_IMETHOD MoveTo(nsIFile *newParentDir, const nsAString& newName) override; \
  NS_IMETHOD MoveToNative(nsIFile *newParentDir, const nsACString& newName) override; \
  NS_IMETHOD MoveToFollowingLinks(nsIFile *newParentDir, const nsAString& newName) override; \
  NS_IMETHOD MoveToFollowingLinksNative(nsIFile *newParentDir, const nsACString& newName) override; \
  NS_IMETHOD RenameTo(nsIFile *newParentDir, const nsAString& newName) override; \
  NS_IMETHOD RenameToNative(nsIFile *newParentDir, const nsACString& newName) override; \
  NS_IMETHOD Remove(bool recursive) override; \
  NS_IMETHOD GetPermissions(uint32_t *aPermissions) override; \
  NS_IMETHOD SetPermissions(uint32_t aPermissions) override; \
  NS_IMETHOD GetPermissionsOfLink(uint32_t *aPermissionsOfLink) override; \
  NS_IMETHOD SetPermissionsOfLink(uint32_t aPermissionsOfLink) override; \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override; \
  NS_IMETHOD SetLastModifiedTime(PRTime aLastModifiedTime) override; \
  NS_IMETHOD GetLastModifiedTimeOfLink(PRTime *aLastModifiedTimeOfLink) override; \
  NS_IMETHOD SetLastModifiedTimeOfLink(PRTime aLastModifiedTimeOfLink) override; \
  NS_IMETHOD GetCreationTime(PRTime *aCreationTime) override; \
  NS_IMETHOD GetCreationTimeOfLink(PRTime *aCreationTimeOfLink) override; \
  NS_IMETHOD GetFileSize(int64_t *aFileSize) override; \
  NS_IMETHOD SetFileSize(int64_t aFileSize) override; \
  NS_IMETHOD GetFileSizeOfLink(int64_t *aFileSizeOfLink) override; \
  NS_IMETHOD GetTarget(nsAString& aTarget) override; \
  NS_IMETHOD GetNativeTarget(nsACString& aNativeTarget) override; \
  NS_IMETHOD GetPath(nsAString& aPath) override; \
  [[nodiscard]] virtual mozilla::PathString NativePath(void) override; \
  NS_IMETHOD Exists(bool *_retval) override; \
  NS_IMETHOD IsWritable(bool *_retval) override; \
  NS_IMETHOD IsReadable(bool *_retval) override; \
  NS_IMETHOD IsExecutable(bool *_retval) override; \
  NS_IMETHOD IsHidden(bool *_retval) override; \
  NS_IMETHOD IsDirectory(bool *_retval) override; \
  NS_IMETHOD IsFile(bool *_retval) override; \
  NS_IMETHOD IsSymlink(bool *_retval) override; \
  NS_IMETHOD IsSpecial(bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD CreateUnique(uint32_t type, uint32_t permissions) override; \
  NS_IMETHOD Clone(nsIFile **_retval) override; \
  NS_IMETHOD Equals(nsIFile *inFile, bool *_retval) override; \
  NS_IMETHOD Contains(nsIFile *inFile, bool *_retval) override; \
  NS_IMETHOD GetParent(nsIFile **aParent) override; \
  NS_IMETHOD GetDirectoryEntriesImpl(nsIDirectoryEnumerator **aDirectoryEntries) override; \
  NS_IMETHOD InitWithPath(const nsAString& filePath) override; \
  NS_IMETHOD InitWithNativePath(const nsACString& filePath) override; \
  NS_IMETHOD InitWithFile(nsIFile *aFile) override; \
  [[nodiscard]] NS_IMETHOD OpenNSPRFileDesc(int32_t flags, int32_t mode, PRFileDesc * * _retval) override; \
  [[nodiscard]] NS_IMETHOD OpenANSIFileDesc(const char * mode, FILE * * _retval) override; \
  [[nodiscard]] NS_IMETHOD Load(PRLibrary * * _retval) override; \
  [[nodiscard]] NS_IMETHOD GetDiskSpaceAvailable(int64_t *aDiskSpaceAvailable) override; \
  NS_IMETHOD AppendRelativePath(const nsAString& relativeFilePath) override; \
  NS_IMETHOD AppendRelativeNativePath(const nsACString& relativeFilePath) override; \
  [[nodiscard]] NS_IMETHOD GetPersistentDescriptor(nsACString& aPersistentDescriptor) override; \
  [[nodiscard]] NS_IMETHOD SetPersistentDescriptor(const nsACString& aPersistentDescriptor) override; \
  [[nodiscard]] NS_IMETHOD Reveal(void) override; \
  [[nodiscard]] NS_IMETHOD Launch(void) override; \
  [[nodiscard]] NS_IMETHOD GetRelativeDescriptor(nsIFile *fromFile, nsACString& _retval) override; \
  [[nodiscard]] NS_IMETHOD SetRelativeDescriptor(nsIFile *fromFile, const nsACString& relativeDesc) override; \
  [[nodiscard]] NS_IMETHOD GetRelativePath(nsIFile *fromFile, nsACString& _retval) override; \
  [[nodiscard]] NS_IMETHOD SetRelativePath(nsIFile *fromFile, const nsACString& relativeDesc) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILE \
  nsresult Append(const nsAString& node); \
  nsresult AppendNative(const nsACString& node); \
  nsresult Normalize(void); \
  [[nodiscard]] nsresult Create(uint32_t type, uint32_t permissions); \
  nsresult GetLeafName(nsAString& aLeafName); \
  nsresult SetLeafName(const nsAString& aLeafName); \
  nsresult GetNativeLeafName(nsACString& aNativeLeafName); \
  nsresult SetNativeLeafName(const nsACString& aNativeLeafName); \
  nsresult CopyTo(nsIFile *newParentDir, const nsAString& newName); \
  nsresult CopyToNative(nsIFile *newParentDir, const nsACString& newName); \
  nsresult CopyToFollowingLinks(nsIFile *newParentDir, const nsAString& newName); \
  nsresult CopyToFollowingLinksNative(nsIFile *newParentDir, const nsACString& newName); \
  nsresult MoveTo(nsIFile *newParentDir, const nsAString& newName); \
  nsresult MoveToNative(nsIFile *newParentDir, const nsACString& newName); \
  nsresult MoveToFollowingLinks(nsIFile *newParentDir, const nsAString& newName); \
  nsresult MoveToFollowingLinksNative(nsIFile *newParentDir, const nsACString& newName); \
  nsresult RenameTo(nsIFile *newParentDir, const nsAString& newName); \
  nsresult RenameToNative(nsIFile *newParentDir, const nsACString& newName); \
  nsresult Remove(bool recursive); \
  nsresult GetPermissions(uint32_t *aPermissions); \
  nsresult SetPermissions(uint32_t aPermissions); \
  nsresult GetPermissionsOfLink(uint32_t *aPermissionsOfLink); \
  nsresult SetPermissionsOfLink(uint32_t aPermissionsOfLink); \
  nsresult GetLastModifiedTime(PRTime *aLastModifiedTime); \
  nsresult SetLastModifiedTime(PRTime aLastModifiedTime); \
  nsresult GetLastModifiedTimeOfLink(PRTime *aLastModifiedTimeOfLink); \
  nsresult SetLastModifiedTimeOfLink(PRTime aLastModifiedTimeOfLink); \
  nsresult GetCreationTime(PRTime *aCreationTime); \
  nsresult GetCreationTimeOfLink(PRTime *aCreationTimeOfLink); \
  nsresult GetFileSize(int64_t *aFileSize); \
  nsresult SetFileSize(int64_t aFileSize); \
  nsresult GetFileSizeOfLink(int64_t *aFileSizeOfLink); \
  nsresult GetTarget(nsAString& aTarget); \
  nsresult GetNativeTarget(nsACString& aNativeTarget); \
  nsresult GetPath(nsAString& aPath); \
  [[nodiscard]] mozilla::PathString NativePath(void); \
  nsresult Exists(bool *_retval); \
  nsresult IsWritable(bool *_retval); \
  nsresult IsReadable(bool *_retval); \
  nsresult IsExecutable(bool *_retval); \
  nsresult IsHidden(bool *_retval); \
  nsresult IsDirectory(bool *_retval); \
  nsresult IsFile(bool *_retval); \
  nsresult IsSymlink(bool *_retval); \
  nsresult IsSpecial(bool *_retval); \
  [[nodiscard]] nsresult CreateUnique(uint32_t type, uint32_t permissions); \
  nsresult Clone(nsIFile **_retval); \
  nsresult Equals(nsIFile *inFile, bool *_retval); \
  nsresult Contains(nsIFile *inFile, bool *_retval); \
  nsresult GetParent(nsIFile **aParent); \
  nsresult GetDirectoryEntriesImpl(nsIDirectoryEnumerator **aDirectoryEntries); \
  nsresult InitWithPath(const nsAString& filePath); \
  nsresult InitWithNativePath(const nsACString& filePath); \
  nsresult InitWithFile(nsIFile *aFile); \
  [[nodiscard]] nsresult OpenNSPRFileDesc(int32_t flags, int32_t mode, PRFileDesc * * _retval); \
  [[nodiscard]] nsresult OpenANSIFileDesc(const char * mode, FILE * * _retval); \
  [[nodiscard]] nsresult Load(PRLibrary * * _retval); \
  [[nodiscard]] nsresult GetDiskSpaceAvailable(int64_t *aDiskSpaceAvailable); \
  nsresult AppendRelativePath(const nsAString& relativeFilePath); \
  nsresult AppendRelativeNativePath(const nsACString& relativeFilePath); \
  [[nodiscard]] nsresult GetPersistentDescriptor(nsACString& aPersistentDescriptor); \
  [[nodiscard]] nsresult SetPersistentDescriptor(const nsACString& aPersistentDescriptor); \
  [[nodiscard]] nsresult Reveal(void); \
  [[nodiscard]] nsresult Launch(void); \
  [[nodiscard]] nsresult GetRelativeDescriptor(nsIFile *fromFile, nsACString& _retval); \
  [[nodiscard]] nsresult SetRelativeDescriptor(nsIFile *fromFile, const nsACString& relativeDesc); \
  [[nodiscard]] nsresult GetRelativePath(nsIFile *fromFile, nsACString& _retval); \
  [[nodiscard]] nsresult SetRelativePath(nsIFile *fromFile, const nsACString& relativeDesc); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILE(_to) \
  NS_IMETHOD Append(const nsAString& node) override { return _to Append(node); } \
  NS_IMETHOD AppendNative(const nsACString& node) override { return _to AppendNative(node); } \
  NS_IMETHOD Normalize(void) override { return _to Normalize(); } \
  [[nodiscard]] NS_IMETHOD Create(uint32_t type, uint32_t permissions) override { return _to Create(type, permissions); } \
  NS_IMETHOD GetLeafName(nsAString& aLeafName) override { return _to GetLeafName(aLeafName); } \
  NS_IMETHOD SetLeafName(const nsAString& aLeafName) override { return _to SetLeafName(aLeafName); } \
  NS_IMETHOD GetNativeLeafName(nsACString& aNativeLeafName) override { return _to GetNativeLeafName(aNativeLeafName); } \
  NS_IMETHOD SetNativeLeafName(const nsACString& aNativeLeafName) override { return _to SetNativeLeafName(aNativeLeafName); } \
  NS_IMETHOD CopyTo(nsIFile *newParentDir, const nsAString& newName) override { return _to CopyTo(newParentDir, newName); } \
  NS_IMETHOD CopyToNative(nsIFile *newParentDir, const nsACString& newName) override { return _to CopyToNative(newParentDir, newName); } \
  NS_IMETHOD CopyToFollowingLinks(nsIFile *newParentDir, const nsAString& newName) override { return _to CopyToFollowingLinks(newParentDir, newName); } \
  NS_IMETHOD CopyToFollowingLinksNative(nsIFile *newParentDir, const nsACString& newName) override { return _to CopyToFollowingLinksNative(newParentDir, newName); } \
  NS_IMETHOD MoveTo(nsIFile *newParentDir, const nsAString& newName) override { return _to MoveTo(newParentDir, newName); } \
  NS_IMETHOD MoveToNative(nsIFile *newParentDir, const nsACString& newName) override { return _to MoveToNative(newParentDir, newName); } \
  NS_IMETHOD MoveToFollowingLinks(nsIFile *newParentDir, const nsAString& newName) override { return _to MoveToFollowingLinks(newParentDir, newName); } \
  NS_IMETHOD MoveToFollowingLinksNative(nsIFile *newParentDir, const nsACString& newName) override { return _to MoveToFollowingLinksNative(newParentDir, newName); } \
  NS_IMETHOD RenameTo(nsIFile *newParentDir, const nsAString& newName) override { return _to RenameTo(newParentDir, newName); } \
  NS_IMETHOD RenameToNative(nsIFile *newParentDir, const nsACString& newName) override { return _to RenameToNative(newParentDir, newName); } \
  NS_IMETHOD Remove(bool recursive) override { return _to Remove(recursive); } \
  NS_IMETHOD GetPermissions(uint32_t *aPermissions) override { return _to GetPermissions(aPermissions); } \
  NS_IMETHOD SetPermissions(uint32_t aPermissions) override { return _to SetPermissions(aPermissions); } \
  NS_IMETHOD GetPermissionsOfLink(uint32_t *aPermissionsOfLink) override { return _to GetPermissionsOfLink(aPermissionsOfLink); } \
  NS_IMETHOD SetPermissionsOfLink(uint32_t aPermissionsOfLink) override { return _to SetPermissionsOfLink(aPermissionsOfLink); } \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override { return _to GetLastModifiedTime(aLastModifiedTime); } \
  NS_IMETHOD SetLastModifiedTime(PRTime aLastModifiedTime) override { return _to SetLastModifiedTime(aLastModifiedTime); } \
  NS_IMETHOD GetLastModifiedTimeOfLink(PRTime *aLastModifiedTimeOfLink) override { return _to GetLastModifiedTimeOfLink(aLastModifiedTimeOfLink); } \
  NS_IMETHOD SetLastModifiedTimeOfLink(PRTime aLastModifiedTimeOfLink) override { return _to SetLastModifiedTimeOfLink(aLastModifiedTimeOfLink); } \
  NS_IMETHOD GetCreationTime(PRTime *aCreationTime) override { return _to GetCreationTime(aCreationTime); } \
  NS_IMETHOD GetCreationTimeOfLink(PRTime *aCreationTimeOfLink) override { return _to GetCreationTimeOfLink(aCreationTimeOfLink); } \
  NS_IMETHOD GetFileSize(int64_t *aFileSize) override { return _to GetFileSize(aFileSize); } \
  NS_IMETHOD SetFileSize(int64_t aFileSize) override { return _to SetFileSize(aFileSize); } \
  NS_IMETHOD GetFileSizeOfLink(int64_t *aFileSizeOfLink) override { return _to GetFileSizeOfLink(aFileSizeOfLink); } \
  NS_IMETHOD GetTarget(nsAString& aTarget) override { return _to GetTarget(aTarget); } \
  NS_IMETHOD GetNativeTarget(nsACString& aNativeTarget) override { return _to GetNativeTarget(aNativeTarget); } \
  NS_IMETHOD GetPath(nsAString& aPath) override { return _to GetPath(aPath); } \
  [[nodiscard]] virtual mozilla::PathString NativePath(void) override { return _to NativePath(); } \
  NS_IMETHOD Exists(bool *_retval) override { return _to Exists(_retval); } \
  NS_IMETHOD IsWritable(bool *_retval) override { return _to IsWritable(_retval); } \
  NS_IMETHOD IsReadable(bool *_retval) override { return _to IsReadable(_retval); } \
  NS_IMETHOD IsExecutable(bool *_retval) override { return _to IsExecutable(_retval); } \
  NS_IMETHOD IsHidden(bool *_retval) override { return _to IsHidden(_retval); } \
  NS_IMETHOD IsDirectory(bool *_retval) override { return _to IsDirectory(_retval); } \
  NS_IMETHOD IsFile(bool *_retval) override { return _to IsFile(_retval); } \
  NS_IMETHOD IsSymlink(bool *_retval) override { return _to IsSymlink(_retval); } \
  NS_IMETHOD IsSpecial(bool *_retval) override { return _to IsSpecial(_retval); } \
  [[nodiscard]] NS_IMETHOD CreateUnique(uint32_t type, uint32_t permissions) override { return _to CreateUnique(type, permissions); } \
  NS_IMETHOD Clone(nsIFile **_retval) override { return _to Clone(_retval); } \
  NS_IMETHOD Equals(nsIFile *inFile, bool *_retval) override { return _to Equals(inFile, _retval); } \
  NS_IMETHOD Contains(nsIFile *inFile, bool *_retval) override { return _to Contains(inFile, _retval); } \
  NS_IMETHOD GetParent(nsIFile **aParent) override { return _to GetParent(aParent); } \
  NS_IMETHOD GetDirectoryEntriesImpl(nsIDirectoryEnumerator **aDirectoryEntries) override { return _to GetDirectoryEntriesImpl(aDirectoryEntries); } \
  NS_IMETHOD InitWithPath(const nsAString& filePath) override { return _to InitWithPath(filePath); } \
  NS_IMETHOD InitWithNativePath(const nsACString& filePath) override { return _to InitWithNativePath(filePath); } \
  NS_IMETHOD InitWithFile(nsIFile *aFile) override { return _to InitWithFile(aFile); } \
  [[nodiscard]] NS_IMETHOD OpenNSPRFileDesc(int32_t flags, int32_t mode, PRFileDesc * * _retval) override { return _to OpenNSPRFileDesc(flags, mode, _retval); } \
  [[nodiscard]] NS_IMETHOD OpenANSIFileDesc(const char * mode, FILE * * _retval) override { return _to OpenANSIFileDesc(mode, _retval); } \
  [[nodiscard]] NS_IMETHOD Load(PRLibrary * * _retval) override { return _to Load(_retval); } \
  [[nodiscard]] NS_IMETHOD GetDiskSpaceAvailable(int64_t *aDiskSpaceAvailable) override { return _to GetDiskSpaceAvailable(aDiskSpaceAvailable); } \
  NS_IMETHOD AppendRelativePath(const nsAString& relativeFilePath) override { return _to AppendRelativePath(relativeFilePath); } \
  NS_IMETHOD AppendRelativeNativePath(const nsACString& relativeFilePath) override { return _to AppendRelativeNativePath(relativeFilePath); } \
  [[nodiscard]] NS_IMETHOD GetPersistentDescriptor(nsACString& aPersistentDescriptor) override { return _to GetPersistentDescriptor(aPersistentDescriptor); } \
  [[nodiscard]] NS_IMETHOD SetPersistentDescriptor(const nsACString& aPersistentDescriptor) override { return _to SetPersistentDescriptor(aPersistentDescriptor); } \
  [[nodiscard]] NS_IMETHOD Reveal(void) override { return _to Reveal(); } \
  [[nodiscard]] NS_IMETHOD Launch(void) override { return _to Launch(); } \
  [[nodiscard]] NS_IMETHOD GetRelativeDescriptor(nsIFile *fromFile, nsACString& _retval) override { return _to GetRelativeDescriptor(fromFile, _retval); } \
  [[nodiscard]] NS_IMETHOD SetRelativeDescriptor(nsIFile *fromFile, const nsACString& relativeDesc) override { return _to SetRelativeDescriptor(fromFile, relativeDesc); } \
  [[nodiscard]] NS_IMETHOD GetRelativePath(nsIFile *fromFile, nsACString& _retval) override { return _to GetRelativePath(fromFile, _retval); } \
  [[nodiscard]] NS_IMETHOD SetRelativePath(nsIFile *fromFile, const nsACString& relativeDesc) override { return _to SetRelativePath(fromFile, relativeDesc); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILE(_to) \
  NS_IMETHOD Append(const nsAString& node) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Append(node); } \
  NS_IMETHOD AppendNative(const nsACString& node) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendNative(node); } \
  NS_IMETHOD Normalize(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Normalize(); } \
  [[nodiscard]] NS_IMETHOD Create(uint32_t type, uint32_t permissions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Create(type, permissions); } \
  NS_IMETHOD GetLeafName(nsAString& aLeafName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLeafName(aLeafName); } \
  NS_IMETHOD SetLeafName(const nsAString& aLeafName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLeafName(aLeafName); } \
  NS_IMETHOD GetNativeLeafName(nsACString& aNativeLeafName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNativeLeafName(aNativeLeafName); } \
  NS_IMETHOD SetNativeLeafName(const nsACString& aNativeLeafName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNativeLeafName(aNativeLeafName); } \
  NS_IMETHOD CopyTo(nsIFile *newParentDir, const nsAString& newName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CopyTo(newParentDir, newName); } \
  NS_IMETHOD CopyToNative(nsIFile *newParentDir, const nsACString& newName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CopyToNative(newParentDir, newName); } \
  NS_IMETHOD CopyToFollowingLinks(nsIFile *newParentDir, const nsAString& newName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CopyToFollowingLinks(newParentDir, newName); } \
  NS_IMETHOD CopyToFollowingLinksNative(nsIFile *newParentDir, const nsACString& newName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CopyToFollowingLinksNative(newParentDir, newName); } \
  NS_IMETHOD MoveTo(nsIFile *newParentDir, const nsAString& newName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveTo(newParentDir, newName); } \
  NS_IMETHOD MoveToNative(nsIFile *newParentDir, const nsACString& newName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveToNative(newParentDir, newName); } \
  NS_IMETHOD MoveToFollowingLinks(nsIFile *newParentDir, const nsAString& newName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveToFollowingLinks(newParentDir, newName); } \
  NS_IMETHOD MoveToFollowingLinksNative(nsIFile *newParentDir, const nsACString& newName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveToFollowingLinksNative(newParentDir, newName); } \
  NS_IMETHOD RenameTo(nsIFile *newParentDir, const nsAString& newName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RenameTo(newParentDir, newName); } \
  NS_IMETHOD RenameToNative(nsIFile *newParentDir, const nsACString& newName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RenameToNative(newParentDir, newName); } \
  NS_IMETHOD Remove(bool recursive) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Remove(recursive); } \
  NS_IMETHOD GetPermissions(uint32_t *aPermissions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPermissions(aPermissions); } \
  NS_IMETHOD SetPermissions(uint32_t aPermissions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPermissions(aPermissions); } \
  NS_IMETHOD GetPermissionsOfLink(uint32_t *aPermissionsOfLink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPermissionsOfLink(aPermissionsOfLink); } \
  NS_IMETHOD SetPermissionsOfLink(uint32_t aPermissionsOfLink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPermissionsOfLink(aPermissionsOfLink); } \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastModifiedTime(aLastModifiedTime); } \
  NS_IMETHOD SetLastModifiedTime(PRTime aLastModifiedTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLastModifiedTime(aLastModifiedTime); } \
  NS_IMETHOD GetLastModifiedTimeOfLink(PRTime *aLastModifiedTimeOfLink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastModifiedTimeOfLink(aLastModifiedTimeOfLink); } \
  NS_IMETHOD SetLastModifiedTimeOfLink(PRTime aLastModifiedTimeOfLink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLastModifiedTimeOfLink(aLastModifiedTimeOfLink); } \
  NS_IMETHOD GetCreationTime(PRTime *aCreationTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCreationTime(aCreationTime); } \
  NS_IMETHOD GetCreationTimeOfLink(PRTime *aCreationTimeOfLink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCreationTimeOfLink(aCreationTimeOfLink); } \
  NS_IMETHOD GetFileSize(int64_t *aFileSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileSize(aFileSize); } \
  NS_IMETHOD SetFileSize(int64_t aFileSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFileSize(aFileSize); } \
  NS_IMETHOD GetFileSizeOfLink(int64_t *aFileSizeOfLink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileSizeOfLink(aFileSizeOfLink); } \
  NS_IMETHOD GetTarget(nsAString& aTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTarget(aTarget); } \
  NS_IMETHOD GetNativeTarget(nsACString& aNativeTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNativeTarget(aNativeTarget); } \
  NS_IMETHOD GetPath(nsAString& aPath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPath(aPath); } \
  [[nodiscard]] virtual mozilla::PathString NativePath(void) override; \
  NS_IMETHOD Exists(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Exists(_retval); } \
  NS_IMETHOD IsWritable(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsWritable(_retval); } \
  NS_IMETHOD IsReadable(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsReadable(_retval); } \
  NS_IMETHOD IsExecutable(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsExecutable(_retval); } \
  NS_IMETHOD IsHidden(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsHidden(_retval); } \
  NS_IMETHOD IsDirectory(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsDirectory(_retval); } \
  NS_IMETHOD IsFile(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsFile(_retval); } \
  NS_IMETHOD IsSymlink(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSymlink(_retval); } \
  NS_IMETHOD IsSpecial(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSpecial(_retval); } \
  [[nodiscard]] NS_IMETHOD CreateUnique(uint32_t type, uint32_t permissions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateUnique(type, permissions); } \
  NS_IMETHOD Clone(nsIFile **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clone(_retval); } \
  NS_IMETHOD Equals(nsIFile *inFile, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Equals(inFile, _retval); } \
  NS_IMETHOD Contains(nsIFile *inFile, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Contains(inFile, _retval); } \
  NS_IMETHOD GetParent(nsIFile **aParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParent(aParent); } \
  NS_IMETHOD GetDirectoryEntriesImpl(nsIDirectoryEnumerator **aDirectoryEntries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDirectoryEntriesImpl(aDirectoryEntries); } \
  NS_IMETHOD InitWithPath(const nsAString& filePath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithPath(filePath); } \
  NS_IMETHOD InitWithNativePath(const nsACString& filePath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithNativePath(filePath); } \
  NS_IMETHOD InitWithFile(nsIFile *aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithFile(aFile); } \
  [[nodiscard]] NS_IMETHOD OpenNSPRFileDesc(int32_t flags, int32_t mode, PRFileDesc * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenNSPRFileDesc(flags, mode, _retval); } \
  [[nodiscard]] NS_IMETHOD OpenANSIFileDesc(const char * mode, FILE * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenANSIFileDesc(mode, _retval); } \
  [[nodiscard]] NS_IMETHOD Load(PRLibrary * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Load(_retval); } \
  [[nodiscard]] NS_IMETHOD GetDiskSpaceAvailable(int64_t *aDiskSpaceAvailable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDiskSpaceAvailable(aDiskSpaceAvailable); } \
  NS_IMETHOD AppendRelativePath(const nsAString& relativeFilePath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendRelativePath(relativeFilePath); } \
  NS_IMETHOD AppendRelativeNativePath(const nsACString& relativeFilePath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendRelativeNativePath(relativeFilePath); } \
  [[nodiscard]] NS_IMETHOD GetPersistentDescriptor(nsACString& aPersistentDescriptor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPersistentDescriptor(aPersistentDescriptor); } \
  [[nodiscard]] NS_IMETHOD SetPersistentDescriptor(const nsACString& aPersistentDescriptor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPersistentDescriptor(aPersistentDescriptor); } \
  [[nodiscard]] NS_IMETHOD Reveal(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reveal(); } \
  [[nodiscard]] NS_IMETHOD Launch(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Launch(); } \
  [[nodiscard]] NS_IMETHOD GetRelativeDescriptor(nsIFile *fromFile, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRelativeDescriptor(fromFile, _retval); } \
  [[nodiscard]] NS_IMETHOD SetRelativeDescriptor(nsIFile *fromFile, const nsACString& relativeDesc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRelativeDescriptor(fromFile, relativeDesc); } \
  [[nodiscard]] NS_IMETHOD GetRelativePath(nsIFile *fromFile, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRelativePath(fromFile, _retval); } \
  [[nodiscard]] NS_IMETHOD SetRelativePath(nsIFile *fromFile, const nsACString& relativeDesc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRelativePath(fromFile, relativeDesc); } 

#ifdef MOZILLA_INTERNAL_API
#include "nsDirectoryServiceUtils.h"
#include "nsString.h"
inline std::ostream& operator<<(std::ostream& aOut, const nsIFile& aFile) {
  nsIFile* file = const_cast<nsIFile*>(&aFile);
  nsAutoString path;
  file->GetPath(path);
  return aOut << "nsIFile { " << path << " }";
}
#endif

#endif /* __gen_nsIFile_h__ */
