/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/nsIJARURI.idl
 */

#ifndef __gen_nsIJARURI_h__
#define __gen_nsIJARURI_h__


#ifndef __gen_nsIURL_h__
#include "nsIURL.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIJARURI */
#define NS_IJARURI_IID_STR "646a508c-f786-4e14-be6d-8dda2a633c60"

#define NS_IJARURI_IID \
  {0x646a508c, 0xf786, 0x4e14, \
    { 0xbe, 0x6d, 0x8d, 0xda, 0x2a, 0x63, 0x3c, 0x60 }}

class NS_NO_VTABLE nsIJARURI : public nsIURL {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IJARURI_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIJARURI;

  /* readonly attribute nsIURI JARFile; */
  NS_IMETHOD GetJARFile(nsIURI **aJARFile) = 0;

  /* readonly attribute AUTF8String JAREntry; */
  NS_IMETHOD GetJAREntry(nsACString& aJAREntry) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIJARURI, NS_IJARURI_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIJARURI \
  NS_IMETHOD GetJARFile(nsIURI **aJARFile) override; \
  NS_IMETHOD GetJAREntry(nsACString& aJAREntry) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIJARURI \
  nsresult GetJARFile(nsIURI **aJARFile); \
  nsresult GetJAREntry(nsACString& aJAREntry); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIJARURI(_to) \
  NS_IMETHOD GetJARFile(nsIURI **aJARFile) override { return _to GetJARFile(aJARFile); } \
  NS_IMETHOD GetJAREntry(nsACString& aJAREntry) override { return _to GetJAREntry(aJAREntry); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIJARURI(_to) \
  NS_IMETHOD GetJARFile(nsIURI **aJARFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetJARFile(aJARFile); } \
  NS_IMETHOD GetJAREntry(nsACString& aJAREntry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetJAREntry(aJAREntry); } 


/* starting interface:    nsIJARURIMutator */
#define NS_IJARURIMUTATOR_IID_STR "d66df117-eda7-4324-b4e4-1f670ff6718e"

#define NS_IJARURIMUTATOR_IID \
  {0xd66df117, 0xeda7, 0x4324, \
    { 0xb4, 0xe4, 0x1f, 0x67, 0x0f, 0xf6, 0x71, 0x8e }}

class NS_NO_VTABLE nsIJARURIMutator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IJARURIMUTATOR_IID)

  /* void setSpecBaseCharset (in AUTF8String aSpec, in nsIURI aBase, in string aCharset); */
  NS_IMETHOD SetSpecBaseCharset(const nsACString& aSpec, nsIURI *aBase, const char * aCharset) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIJARURIMutator, NS_IJARURIMUTATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIJARURIMUTATOR \
  NS_IMETHOD SetSpecBaseCharset(const nsACString& aSpec, nsIURI *aBase, const char * aCharset) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIJARURIMUTATOR \
  nsresult SetSpecBaseCharset(const nsACString& aSpec, nsIURI *aBase, const char * aCharset); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIJARURIMUTATOR(_to) \
  NS_IMETHOD SetSpecBaseCharset(const nsACString& aSpec, nsIURI *aBase, const char * aCharset) override { return _to SetSpecBaseCharset(aSpec, aBase, aCharset); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIJARURIMUTATOR(_to) \
  NS_IMETHOD SetSpecBaseCharset(const nsACString& aSpec, nsIURI *aBase, const char * aCharset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSpecBaseCharset(aSpec, aBase, aCharset); } 


#endif /* __gen_nsIJARURI_h__ */
