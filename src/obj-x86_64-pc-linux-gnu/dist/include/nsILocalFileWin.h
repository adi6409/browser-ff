/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsILocalFileWin.idl
 */

#ifndef __gen_nsILocalFileWin_h__
#define __gen_nsILocalFileWin_h__


#ifndef __gen_nsIFile_h__
#include "nsIFile.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
struct PRFileDesc;

/* starting interface:    nsILocalFileWin */
#define NS_ILOCALFILEWIN_IID_STR "e7a3a954-384b-4aeb-a5f7-55626b0de9be"

#define NS_ILOCALFILEWIN_IID \
  {0xe7a3a954, 0x384b, 0x4aeb, \
    { 0xa5, 0xf7, 0x55, 0x62, 0x6b, 0x0d, 0xe9, 0xbe }}

class NS_NO_VTABLE nsILocalFileWin : public nsIFile {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOCALFILEWIN_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILocalFileWin;

  /* void initWithCommandLine (in AString aCommandLine); */
  NS_IMETHOD InitWithCommandLine(const nsAString& aCommandLine) = 0;

  /* AString getVersionInfoField (in string aField); */
  NS_IMETHOD GetVersionInfoField(const char * aField, nsAString& _retval) = 0;

  /* readonly attribute AString canonicalPath; */
  NS_IMETHOD GetCanonicalPath(nsAString& aCanonicalPath) = 0;

  /* [noscript] readonly attribute ACString nativeCanonicalPath; */
  NS_IMETHOD GetNativeCanonicalPath(nsACString& aNativeCanonicalPath) = 0;

  enum {
    WFA_SEARCH_INDEXED = 1U,
    WFA_READONLY = 2U,
    WFA_READWRITE = 4U
  };

  /* attribute unsigned long fileAttributesWin; */
  NS_IMETHOD GetFileAttributesWin(uint32_t *aFileAttributesWin) = 0;
  NS_IMETHOD SetFileAttributesWin(uint32_t aFileAttributesWin) = 0;

  /* attribute boolean useDOSDevicePathSyntax; */
  NS_IMETHOD GetUseDOSDevicePathSyntax(bool *aUseDOSDevicePathSyntax) = 0;
  NS_IMETHOD SetUseDOSDevicePathSyntax(bool aUseDOSDevicePathSyntax) = 0;

  /* [noscript] PRFileDescStar openNSPRFileDescShareDelete (in long flags, in long mode); */
  NS_IMETHOD OpenNSPRFileDescShareDelete(int32_t flags, int32_t mode, PRFileDesc * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILocalFileWin, NS_ILOCALFILEWIN_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOCALFILEWIN \
  NS_IMETHOD InitWithCommandLine(const nsAString& aCommandLine) override; \
  NS_IMETHOD GetVersionInfoField(const char * aField, nsAString& _retval) override; \
  NS_IMETHOD GetCanonicalPath(nsAString& aCanonicalPath) override; \
  NS_IMETHOD GetNativeCanonicalPath(nsACString& aNativeCanonicalPath) override; \
  NS_IMETHOD GetFileAttributesWin(uint32_t *aFileAttributesWin) override; \
  NS_IMETHOD SetFileAttributesWin(uint32_t aFileAttributesWin) override; \
  NS_IMETHOD GetUseDOSDevicePathSyntax(bool *aUseDOSDevicePathSyntax) override; \
  NS_IMETHOD SetUseDOSDevicePathSyntax(bool aUseDOSDevicePathSyntax) override; \
  NS_IMETHOD OpenNSPRFileDescShareDelete(int32_t flags, int32_t mode, PRFileDesc * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOCALFILEWIN \
  nsresult InitWithCommandLine(const nsAString& aCommandLine); \
  nsresult GetVersionInfoField(const char * aField, nsAString& _retval); \
  nsresult GetCanonicalPath(nsAString& aCanonicalPath); \
  nsresult GetNativeCanonicalPath(nsACString& aNativeCanonicalPath); \
  nsresult GetFileAttributesWin(uint32_t *aFileAttributesWin); \
  nsresult SetFileAttributesWin(uint32_t aFileAttributesWin); \
  nsresult GetUseDOSDevicePathSyntax(bool *aUseDOSDevicePathSyntax); \
  nsresult SetUseDOSDevicePathSyntax(bool aUseDOSDevicePathSyntax); \
  nsresult OpenNSPRFileDescShareDelete(int32_t flags, int32_t mode, PRFileDesc * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOCALFILEWIN(_to) \
  NS_IMETHOD InitWithCommandLine(const nsAString& aCommandLine) override { return _to InitWithCommandLine(aCommandLine); } \
  NS_IMETHOD GetVersionInfoField(const char * aField, nsAString& _retval) override { return _to GetVersionInfoField(aField, _retval); } \
  NS_IMETHOD GetCanonicalPath(nsAString& aCanonicalPath) override { return _to GetCanonicalPath(aCanonicalPath); } \
  NS_IMETHOD GetNativeCanonicalPath(nsACString& aNativeCanonicalPath) override { return _to GetNativeCanonicalPath(aNativeCanonicalPath); } \
  NS_IMETHOD GetFileAttributesWin(uint32_t *aFileAttributesWin) override { return _to GetFileAttributesWin(aFileAttributesWin); } \
  NS_IMETHOD SetFileAttributesWin(uint32_t aFileAttributesWin) override { return _to SetFileAttributesWin(aFileAttributesWin); } \
  NS_IMETHOD GetUseDOSDevicePathSyntax(bool *aUseDOSDevicePathSyntax) override { return _to GetUseDOSDevicePathSyntax(aUseDOSDevicePathSyntax); } \
  NS_IMETHOD SetUseDOSDevicePathSyntax(bool aUseDOSDevicePathSyntax) override { return _to SetUseDOSDevicePathSyntax(aUseDOSDevicePathSyntax); } \
  NS_IMETHOD OpenNSPRFileDescShareDelete(int32_t flags, int32_t mode, PRFileDesc * * _retval) override { return _to OpenNSPRFileDescShareDelete(flags, mode, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOCALFILEWIN(_to) \
  NS_IMETHOD InitWithCommandLine(const nsAString& aCommandLine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithCommandLine(aCommandLine); } \
  NS_IMETHOD GetVersionInfoField(const char * aField, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVersionInfoField(aField, _retval); } \
  NS_IMETHOD GetCanonicalPath(nsAString& aCanonicalPath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanonicalPath(aCanonicalPath); } \
  NS_IMETHOD GetNativeCanonicalPath(nsACString& aNativeCanonicalPath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNativeCanonicalPath(aNativeCanonicalPath); } \
  NS_IMETHOD GetFileAttributesWin(uint32_t *aFileAttributesWin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileAttributesWin(aFileAttributesWin); } \
  NS_IMETHOD SetFileAttributesWin(uint32_t aFileAttributesWin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFileAttributesWin(aFileAttributesWin); } \
  NS_IMETHOD GetUseDOSDevicePathSyntax(bool *aUseDOSDevicePathSyntax) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUseDOSDevicePathSyntax(aUseDOSDevicePathSyntax); } \
  NS_IMETHOD SetUseDOSDevicePathSyntax(bool aUseDOSDevicePathSyntax) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUseDOSDevicePathSyntax(aUseDOSDevicePathSyntax); } \
  NS_IMETHOD OpenNSPRFileDescShareDelete(int32_t flags, int32_t mode, PRFileDesc * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenNSPRFileDescShareDelete(flags, mode, _retval); } 


#endif /* __gen_nsILocalFileWin_h__ */
