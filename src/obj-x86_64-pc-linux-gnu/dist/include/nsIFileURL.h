/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIFileURL.idl
 */

#ifndef __gen_nsIFileURL_h__
#define __gen_nsIFileURL_h__


#ifndef __gen_nsIURL_h__
#include "nsIURL.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIURIMutator; /* forward declaration */


/* starting interface:    nsIFileURL */
#define NS_IFILEURL_IID_STR "e91ac988-27c2-448b-b1a1-3822e1ef1987"

#define NS_IFILEURL_IID \
  {0xe91ac988, 0x27c2, 0x448b, \
    { 0xb1, 0xa1, 0x38, 0x22, 0xe1, 0xef, 0x19, 0x87 }}

class NS_NO_VTABLE nsIFileURL : public nsIURL {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILEURL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFileURL;

  /* readonly attribute nsIFile file; */
  NS_IMETHOD GetFile(nsIFile **aFile) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFileURL, NS_IFILEURL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILEURL \
  NS_IMETHOD GetFile(nsIFile **aFile) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILEURL \
  nsresult GetFile(nsIFile **aFile); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILEURL(_to) \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return _to GetFile(aFile); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILEURL(_to) \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFile(aFile); } 


/* starting interface:    nsIFileURLMutator */
#define NS_IFILEURLMUTATOR_IID_STR "a588b6f2-d2b9-4024-84c7-be3368546b57"

#define NS_IFILEURLMUTATOR_IID \
  {0xa588b6f2, 0xd2b9, 0x4024, \
    { 0x84, 0xc7, 0xbe, 0x33, 0x68, 0x54, 0x6b, 0x57 }}

class NS_NO_VTABLE nsIFileURLMutator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILEURLMUTATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFileURLMutator;

  /* [must_use,noscript] void markFileURL (); */
  [[nodiscard]] NS_IMETHOD MarkFileURL(void) = 0;

  /* [must_use,noscript] void setFile (in nsIFile aFile); */
  [[nodiscard]] NS_IMETHOD SetFile(nsIFile *aFile) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFileURLMutator, NS_IFILEURLMUTATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILEURLMUTATOR \
  [[nodiscard]] NS_IMETHOD MarkFileURL(void) override; \
  [[nodiscard]] NS_IMETHOD SetFile(nsIFile *aFile) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILEURLMUTATOR \
  [[nodiscard]] nsresult MarkFileURL(void); \
  [[nodiscard]] nsresult SetFile(nsIFile *aFile); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILEURLMUTATOR(_to) \
  [[nodiscard]] NS_IMETHOD MarkFileURL(void) override { return _to MarkFileURL(); } \
  [[nodiscard]] NS_IMETHOD SetFile(nsIFile *aFile) override { return _to SetFile(aFile); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILEURLMUTATOR(_to) \
  [[nodiscard]] NS_IMETHOD MarkFileURL(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MarkFileURL(); } \
  [[nodiscard]] NS_IMETHOD SetFile(nsIFile *aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFile(aFile); } 


#endif /* __gen_nsIFileURL_h__ */
