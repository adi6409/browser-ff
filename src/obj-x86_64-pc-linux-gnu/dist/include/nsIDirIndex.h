/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsIDirIndex.idl
 */

#ifndef __gen_nsIDirIndex_h__
#define __gen_nsIDirIndex_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDirIndex */
#define NS_IDIRINDEX_IID_STR "23bbabd0-1dd2-11b2-86b7-aad68ae7d7e0"

#define NS_IDIRINDEX_IID \
  {0x23bbabd0, 0x1dd2, 0x11b2, \
    { 0x86, 0xb7, 0xaa, 0xd6, 0x8a, 0xe7, 0xd7, 0xe0 }}

class NS_NO_VTABLE nsIDirIndex : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDIRINDEX_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDirIndex;

  enum {
    TYPE_UNKNOWN = 0U,
    TYPE_DIRECTORY = 1U,
    TYPE_FILE = 2U,
    TYPE_SYMLINK = 3U
  };

  /* attribute unsigned long type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(uint32_t *aType) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetType(uint32_t aType) = 0;

  /* attribute ACString contentType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentType(nsACString& aContentType) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetContentType(const nsACString& aContentType) = 0;

  /* attribute ACString location; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLocation(nsACString& aLocation) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLocation(const nsACString& aLocation) = 0;

  /* attribute AString description; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDescription(nsAString& aDescription) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDescription(const nsAString& aDescription) = 0;

  /* attribute long long size; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSize(int64_t *aSize) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSize(int64_t aSize) = 0;

  /* attribute PRTime lastModified; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastModified(PRTime *aLastModified) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLastModified(PRTime aLastModified) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDirIndex, NS_IDIRINDEX_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDIRINDEX \
  NS_IMETHOD GetType(uint32_t *aType) override; \
  NS_IMETHOD SetType(uint32_t aType) override; \
  NS_IMETHOD GetContentType(nsACString& aContentType) override; \
  NS_IMETHOD SetContentType(const nsACString& aContentType) override; \
  NS_IMETHOD GetLocation(nsACString& aLocation) override; \
  NS_IMETHOD SetLocation(const nsACString& aLocation) override; \
  NS_IMETHOD GetDescription(nsAString& aDescription) override; \
  NS_IMETHOD SetDescription(const nsAString& aDescription) override; \
  NS_IMETHOD GetSize(int64_t *aSize) override; \
  NS_IMETHOD SetSize(int64_t aSize) override; \
  NS_IMETHOD GetLastModified(PRTime *aLastModified) override; \
  NS_IMETHOD SetLastModified(PRTime aLastModified) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDIRINDEX \
  nsresult GetType(uint32_t *aType); \
  nsresult SetType(uint32_t aType); \
  nsresult GetContentType(nsACString& aContentType); \
  nsresult SetContentType(const nsACString& aContentType); \
  nsresult GetLocation(nsACString& aLocation); \
  nsresult SetLocation(const nsACString& aLocation); \
  nsresult GetDescription(nsAString& aDescription); \
  nsresult SetDescription(const nsAString& aDescription); \
  nsresult GetSize(int64_t *aSize); \
  nsresult SetSize(int64_t aSize); \
  nsresult GetLastModified(PRTime *aLastModified); \
  nsresult SetLastModified(PRTime aLastModified); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDIRINDEX(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return _to GetType(aType); } \
  NS_IMETHOD SetType(uint32_t aType) override { return _to SetType(aType); } \
  NS_IMETHOD GetContentType(nsACString& aContentType) override { return _to GetContentType(aContentType); } \
  NS_IMETHOD SetContentType(const nsACString& aContentType) override { return _to SetContentType(aContentType); } \
  NS_IMETHOD GetLocation(nsACString& aLocation) override { return _to GetLocation(aLocation); } \
  NS_IMETHOD SetLocation(const nsACString& aLocation) override { return _to SetLocation(aLocation); } \
  NS_IMETHOD GetDescription(nsAString& aDescription) override { return _to GetDescription(aDescription); } \
  NS_IMETHOD SetDescription(const nsAString& aDescription) override { return _to SetDescription(aDescription); } \
  NS_IMETHOD GetSize(int64_t *aSize) override { return _to GetSize(aSize); } \
  NS_IMETHOD SetSize(int64_t aSize) override { return _to SetSize(aSize); } \
  NS_IMETHOD GetLastModified(PRTime *aLastModified) override { return _to GetLastModified(aLastModified); } \
  NS_IMETHOD SetLastModified(PRTime aLastModified) override { return _to SetLastModified(aLastModified); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDIRINDEX(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD SetType(uint32_t aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetType(aType); } \
  NS_IMETHOD GetContentType(nsACString& aContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentType(aContentType); } \
  NS_IMETHOD SetContentType(const nsACString& aContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentType(aContentType); } \
  NS_IMETHOD GetLocation(nsACString& aLocation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLocation(aLocation); } \
  NS_IMETHOD SetLocation(const nsACString& aLocation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLocation(aLocation); } \
  NS_IMETHOD GetDescription(nsAString& aDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDescription(aDescription); } \
  NS_IMETHOD SetDescription(const nsAString& aDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDescription(aDescription); } \
  NS_IMETHOD GetSize(int64_t *aSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSize(aSize); } \
  NS_IMETHOD SetSize(int64_t aSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSize(aSize); } \
  NS_IMETHOD GetLastModified(PRTime *aLastModified) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastModified(aLastModified); } \
  NS_IMETHOD SetLastModified(PRTime aLastModified) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLastModified(aLastModified); } 


#endif /* __gen_nsIDirIndex_h__ */
