/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libpref/nsIRelativeFilePref.idl
 */

#ifndef __gen_nsIRelativeFilePref_h__
#define __gen_nsIRelativeFilePref_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */


/* starting interface:    nsIRelativeFilePref */
#define NS_IRELATIVEFILEPREF_IID_STR "2f977d4e-5485-11d4-87e2-0010a4e75ef2"

#define NS_IRELATIVEFILEPREF_IID \
  {0x2f977d4e, 0x5485, 0x11d4, \
    { 0x87, 0xe2, 0x00, 0x10, 0xa4, 0xe7, 0x5e, 0xf2 }}

class NS_NO_VTABLE nsIRelativeFilePref : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IRELATIVEFILEPREF_IID)

  /* attribute nsIFile file; */
  NS_IMETHOD GetFile(nsIFile **aFile) = 0;
  NS_IMETHOD SetFile(nsIFile *aFile) = 0;

  /* attribute ACString relativeToKey; */
  NS_IMETHOD GetRelativeToKey(nsACString& aRelativeToKey) = 0;
  NS_IMETHOD SetRelativeToKey(const nsACString& aRelativeToKey) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRelativeFilePref, NS_IRELATIVEFILEPREF_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIRELATIVEFILEPREF \
  NS_IMETHOD GetFile(nsIFile **aFile) override; \
  NS_IMETHOD SetFile(nsIFile *aFile) override; \
  NS_IMETHOD GetRelativeToKey(nsACString& aRelativeToKey) override; \
  NS_IMETHOD SetRelativeToKey(const nsACString& aRelativeToKey) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIRELATIVEFILEPREF \
  nsresult GetFile(nsIFile **aFile); \
  nsresult SetFile(nsIFile *aFile); \
  nsresult GetRelativeToKey(nsACString& aRelativeToKey); \
  nsresult SetRelativeToKey(const nsACString& aRelativeToKey); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIRELATIVEFILEPREF(_to) \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return _to GetFile(aFile); } \
  NS_IMETHOD SetFile(nsIFile *aFile) override { return _to SetFile(aFile); } \
  NS_IMETHOD GetRelativeToKey(nsACString& aRelativeToKey) override { return _to GetRelativeToKey(aRelativeToKey); } \
  NS_IMETHOD SetRelativeToKey(const nsACString& aRelativeToKey) override { return _to SetRelativeToKey(aRelativeToKey); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIRELATIVEFILEPREF(_to) \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFile(aFile); } \
  NS_IMETHOD SetFile(nsIFile *aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFile(aFile); } \
  NS_IMETHOD GetRelativeToKey(nsACString& aRelativeToKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRelativeToKey(aRelativeToKey); } \
  NS_IMETHOD SetRelativeToKey(const nsACString& aRelativeToKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRelativeToKey(aRelativeToKey); } 


#endif /* __gen_nsIRelativeFilePref_h__ */
