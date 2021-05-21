/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURL.idl
 */

#ifndef __gen_nsIURL_h__
#define __gen_nsIURL_h__


#ifndef __gen_nsIURI_h__
#include "nsIURI.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURIMutator; /* forward declaration */


/* starting interface:    nsIURL */
#define NS_IURL_IID_STR "86adcd89-0b70-47a2-b0fe-5bb2c5f37e31"

#define NS_IURL_IID \
  {0x86adcd89, 0x0b70, 0x47a2, \
    { 0xb0, 0xfe, 0x5b, 0xb2, 0xc5, 0xf3, 0x7e, 0x31 }}

class NS_NO_VTABLE nsIURL : public nsIURI {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURL;

  /* readonly attribute AUTF8String directory; */
  NS_IMETHOD GetDirectory(nsACString& aDirectory) = 0;

  /* readonly attribute AUTF8String fileName; */
  NS_IMETHOD GetFileName(nsACString& aFileName) = 0;

  /* readonly attribute AUTF8String fileBaseName; */
  NS_IMETHOD GetFileBaseName(nsACString& aFileBaseName) = 0;

  /* readonly attribute AUTF8String fileExtension; */
  NS_IMETHOD GetFileExtension(nsACString& aFileExtension) = 0;

  /* AUTF8String getCommonBaseSpec (in nsIURI aURIToCompare); */
  NS_IMETHOD GetCommonBaseSpec(nsIURI *aURIToCompare, nsACString& _retval) = 0;

  /* AUTF8String getRelativeSpec (in nsIURI aURIToCompare); */
  NS_IMETHOD GetRelativeSpec(nsIURI *aURIToCompare, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURL, NS_IURL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURL \
  NS_IMETHOD GetDirectory(nsACString& aDirectory) override; \
  NS_IMETHOD GetFileName(nsACString& aFileName) override; \
  NS_IMETHOD GetFileBaseName(nsACString& aFileBaseName) override; \
  NS_IMETHOD GetFileExtension(nsACString& aFileExtension) override; \
  NS_IMETHOD GetCommonBaseSpec(nsIURI *aURIToCompare, nsACString& _retval) override; \
  NS_IMETHOD GetRelativeSpec(nsIURI *aURIToCompare, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURL \
  nsresult GetDirectory(nsACString& aDirectory); \
  nsresult GetFileName(nsACString& aFileName); \
  nsresult GetFileBaseName(nsACString& aFileBaseName); \
  nsresult GetFileExtension(nsACString& aFileExtension); \
  nsresult GetCommonBaseSpec(nsIURI *aURIToCompare, nsACString& _retval); \
  nsresult GetRelativeSpec(nsIURI *aURIToCompare, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURL(_to) \
  NS_IMETHOD GetDirectory(nsACString& aDirectory) override { return _to GetDirectory(aDirectory); } \
  NS_IMETHOD GetFileName(nsACString& aFileName) override { return _to GetFileName(aFileName); } \
  NS_IMETHOD GetFileBaseName(nsACString& aFileBaseName) override { return _to GetFileBaseName(aFileBaseName); } \
  NS_IMETHOD GetFileExtension(nsACString& aFileExtension) override { return _to GetFileExtension(aFileExtension); } \
  NS_IMETHOD GetCommonBaseSpec(nsIURI *aURIToCompare, nsACString& _retval) override { return _to GetCommonBaseSpec(aURIToCompare, _retval); } \
  NS_IMETHOD GetRelativeSpec(nsIURI *aURIToCompare, nsACString& _retval) override { return _to GetRelativeSpec(aURIToCompare, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURL(_to) \
  NS_IMETHOD GetDirectory(nsACString& aDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDirectory(aDirectory); } \
  NS_IMETHOD GetFileName(nsACString& aFileName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileName(aFileName); } \
  NS_IMETHOD GetFileBaseName(nsACString& aFileBaseName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileBaseName(aFileBaseName); } \
  NS_IMETHOD GetFileExtension(nsACString& aFileExtension) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileExtension(aFileExtension); } \
  NS_IMETHOD GetCommonBaseSpec(nsIURI *aURIToCompare, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCommonBaseSpec(aURIToCompare, _retval); } \
  NS_IMETHOD GetRelativeSpec(nsIURI *aURIToCompare, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRelativeSpec(aURIToCompare, _retval); } 


/* starting interface:    nsIURLMutator */
#define NS_IURLMUTATOR_IID_STR "25072eb8-f1e6-482f-9ca9-eddd3d65169a"

#define NS_IURLMUTATOR_IID \
  {0x25072eb8, 0xf1e6, 0x482f, \
    { 0x9c, 0xa9, 0xed, 0xdd, 0x3d, 0x65, 0x16, 0x9a }}

class NS_NO_VTABLE nsIURLMutator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLMUTATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURLMutator;

  /* [must_use] nsIURIMutator setFileName (in AUTF8String aFileName); */
  [[nodiscard]] NS_IMETHOD SetFileName(const nsACString& aFileName, nsIURIMutator **_retval) = 0;

  /* [must_use] nsIURIMutator setFileBaseName (in AUTF8String aFileBaseName); */
  [[nodiscard]] NS_IMETHOD SetFileBaseName(const nsACString& aFileBaseName, nsIURIMutator **_retval) = 0;

  /* [must_use] nsIURIMutator setFileExtension (in AUTF8String aFileExtension); */
  [[nodiscard]] NS_IMETHOD SetFileExtension(const nsACString& aFileExtension, nsIURIMutator **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURLMutator, NS_IURLMUTATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLMUTATOR \
  [[nodiscard]] NS_IMETHOD SetFileName(const nsACString& aFileName, nsIURIMutator **_retval) override; \
  [[nodiscard]] NS_IMETHOD SetFileBaseName(const nsACString& aFileBaseName, nsIURIMutator **_retval) override; \
  [[nodiscard]] NS_IMETHOD SetFileExtension(const nsACString& aFileExtension, nsIURIMutator **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLMUTATOR \
  [[nodiscard]] nsresult SetFileName(const nsACString& aFileName, nsIURIMutator **_retval); \
  [[nodiscard]] nsresult SetFileBaseName(const nsACString& aFileBaseName, nsIURIMutator **_retval); \
  [[nodiscard]] nsresult SetFileExtension(const nsACString& aFileExtension, nsIURIMutator **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLMUTATOR(_to) \
  [[nodiscard]] NS_IMETHOD SetFileName(const nsACString& aFileName, nsIURIMutator **_retval) override { return _to SetFileName(aFileName, _retval); } \
  [[nodiscard]] NS_IMETHOD SetFileBaseName(const nsACString& aFileBaseName, nsIURIMutator **_retval) override { return _to SetFileBaseName(aFileBaseName, _retval); } \
  [[nodiscard]] NS_IMETHOD SetFileExtension(const nsACString& aFileExtension, nsIURIMutator **_retval) override { return _to SetFileExtension(aFileExtension, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLMUTATOR(_to) \
  [[nodiscard]] NS_IMETHOD SetFileName(const nsACString& aFileName, nsIURIMutator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFileName(aFileName, _retval); } \
  [[nodiscard]] NS_IMETHOD SetFileBaseName(const nsACString& aFileBaseName, nsIURIMutator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFileBaseName(aFileBaseName, _retval); } \
  [[nodiscard]] NS_IMETHOD SetFileExtension(const nsACString& aFileExtension, nsIURIMutator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFileExtension(aFileExtension, _retval); } 


#endif /* __gen_nsIURL_h__ */
