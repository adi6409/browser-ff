/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsICategoryManager.idl
 */

#ifndef __gen_nsICategoryManager_h__
#define __gen_nsICategoryManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsISupportsPrimitives_h__
#include "nsISupportsPrimitives.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsISimpleEnumerator; /* forward declaration */

#include "nsString.h"

/* starting interface:    nsICategoryEntry */
#define NS_ICATEGORYENTRY_IID_STR "de021d54-57a3-4025-ae63-4c8eedbe74c0"

#define NS_ICATEGORYENTRY_IID \
  {0xde021d54, 0x57a3, 0x4025, \
    { 0xae, 0x63, 0x4c, 0x8e, 0xed, 0xbe, 0x74, 0xc0 }}

class NS_NO_VTABLE nsICategoryEntry : public nsISupportsCString {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICATEGORYENTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICategoryEntry;

  /* readonly attribute ACString entry; */
  NS_IMETHOD GetEntry(nsACString& aEntry) = 0;

  /* readonly attribute ACString value; */
  NS_IMETHOD GetValue(nsACString& aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICategoryEntry, NS_ICATEGORYENTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICATEGORYENTRY \
  NS_IMETHOD GetEntry(nsACString& aEntry) override; \
  NS_IMETHOD GetValue(nsACString& aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICATEGORYENTRY \
  nsresult GetEntry(nsACString& aEntry); \
  nsresult GetValue(nsACString& aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICATEGORYENTRY(_to) \
  NS_IMETHOD GetEntry(nsACString& aEntry) override { return _to GetEntry(aEntry); } \
  NS_IMETHOD GetValue(nsACString& aValue) override { return _to GetValue(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICATEGORYENTRY(_to) \
  NS_IMETHOD GetEntry(nsACString& aEntry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntry(aEntry); } \
  NS_IMETHOD GetValue(nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } 


/* starting interface:    nsICategoryManager */
#define NS_ICATEGORYMANAGER_IID_STR "3275b2cd-af6d-429a-80d7-f0c5120342ac"

#define NS_ICATEGORYMANAGER_IID \
  {0x3275b2cd, 0xaf6d, 0x429a, \
    { 0x80, 0xd7, 0xf0, 0xc5, 0x12, 0x03, 0x42, 0xac }}

class nsICategoryManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICATEGORYMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICategoryManager;

  /* ACString getCategoryEntry (in ACString aCategory, in ACString aEntry); */
  NS_IMETHOD GetCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, nsACString& _retval) = 0;

  /* ACString addCategoryEntry (in ACString aCategory, in ACString aEntry, in ACString aValue, in boolean aPersist, in boolean aReplace); */
  NS_IMETHOD AddCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, const nsACString& aValue, bool aPersist, bool aReplace, nsACString& _retval) = 0;

  /* void deleteCategoryEntry (in ACString aCategory, in ACString aEntry, in boolean aPersist); */
  NS_IMETHOD DeleteCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, bool aPersist) = 0;

  /* void deleteCategory (in ACString aCategory); */
  NS_IMETHOD DeleteCategory(const nsACString& aCategory) = 0;

  /* nsISimpleEnumerator enumerateCategory (in ACString aCategory); */
  NS_IMETHOD EnumerateCategory(const nsACString& aCategory, nsISimpleEnumerator **_retval) = 0;

  /* nsISimpleEnumerator enumerateCategories (); */
  NS_IMETHOD EnumerateCategories(nsISimpleEnumerator **_retval) = 0;

     template<size_t N>
    nsresult
    GetCategoryEntry(const char (&aCategory)[N], const nsACString& aEntry,
                     nsACString& aResult)
    {
        return GetCategoryEntry(nsLiteralCString(aCategory),
                                aEntry, aResult);
    }
    template<size_t N, size_t M>
    nsresult
    GetCategoryEntry(const char (&aCategory)[N], const char (&aEntry)[M],
                     nsACString& aResult)
    {
        return GetCategoryEntry(nsLiteralCString(aCategory),
                                nsLiteralCString(aEntry),
                                aResult);
    }
    nsresult
    AddCategoryEntry(const nsACString& aCategory, const nsACString& aEntry,
                     const nsACString& aValue, bool aPersist, bool aReplace)
    {
        nsCString oldValue;
        return AddCategoryEntry(aCategory, aEntry, aValue, aPersist, aReplace,
                                oldValue);
    }
    template<size_t N>
    nsresult
    AddCategoryEntry(const char (&aCategory)[N], const nsACString& aEntry,
                     const nsACString& aValue, bool aPersist, bool aReplace)
    {
        nsCString oldValue;
        return AddCategoryEntry(nsLiteralCString(aCategory), aEntry, aValue,
                                aPersist, aReplace, oldValue);
    }
    template<size_t N>
    nsresult
    DeleteCategoryEntry(const char (&aCategory)[N], const nsACString& aEntry, bool aPersist)
    {
        return DeleteCategoryEntry(nsLiteralCString(aCategory), aEntry, aPersist);
    }
    template<size_t N>
    nsresult
    EnumerateCategory(const char (&aCategory)[N], nsISimpleEnumerator** aResult)
    {
        return EnumerateCategory(nsLiteralCString(aCategory), aResult);
    }
    };

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICategoryManager, NS_ICATEGORYMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICATEGORYMANAGER \
  NS_IMETHOD GetCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, nsACString& _retval) override; \
  NS_IMETHOD AddCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, const nsACString& aValue, bool aPersist, bool aReplace, nsACString& _retval) override; \
  NS_IMETHOD DeleteCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, bool aPersist) override; \
  NS_IMETHOD DeleteCategory(const nsACString& aCategory) override; \
  NS_IMETHOD EnumerateCategory(const nsACString& aCategory, nsISimpleEnumerator **_retval) override; \
  NS_IMETHOD EnumerateCategories(nsISimpleEnumerator **_retval) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICATEGORYMANAGER \
  nsresult GetCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, nsACString& _retval); \
  nsresult AddCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, const nsACString& aValue, bool aPersist, bool aReplace, nsACString& _retval); \
  nsresult DeleteCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, bool aPersist); \
  nsresult DeleteCategory(const nsACString& aCategory); \
  nsresult EnumerateCategory(const nsACString& aCategory, nsISimpleEnumerator **_retval); \
  nsresult EnumerateCategories(nsISimpleEnumerator **_retval); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICATEGORYMANAGER(_to) \
  NS_IMETHOD GetCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, nsACString& _retval) override { return _to GetCategoryEntry(aCategory, aEntry, _retval); } \
  NS_IMETHOD AddCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, const nsACString& aValue, bool aPersist, bool aReplace, nsACString& _retval) override { return _to AddCategoryEntry(aCategory, aEntry, aValue, aPersist, aReplace, _retval); } \
  NS_IMETHOD DeleteCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, bool aPersist) override { return _to DeleteCategoryEntry(aCategory, aEntry, aPersist); } \
  NS_IMETHOD DeleteCategory(const nsACString& aCategory) override { return _to DeleteCategory(aCategory); } \
  NS_IMETHOD EnumerateCategory(const nsACString& aCategory, nsISimpleEnumerator **_retval) override { return _to EnumerateCategory(aCategory, _retval); } \
  NS_IMETHOD EnumerateCategories(nsISimpleEnumerator **_retval) override { return _to EnumerateCategories(_retval); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICATEGORYMANAGER(_to) \
  NS_IMETHOD GetCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCategoryEntry(aCategory, aEntry, _retval); } \
  NS_IMETHOD AddCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, const nsACString& aValue, bool aPersist, bool aReplace, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddCategoryEntry(aCategory, aEntry, aValue, aPersist, aReplace, _retval); } \
  NS_IMETHOD DeleteCategoryEntry(const nsACString& aCategory, const nsACString& aEntry, bool aPersist) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteCategoryEntry(aCategory, aEntry, aPersist); } \
  NS_IMETHOD DeleteCategory(const nsACString& aCategory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteCategory(aCategory); } \
  NS_IMETHOD EnumerateCategory(const nsACString& aCategory, nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnumerateCategory(aCategory, _retval); } \
  NS_IMETHOD EnumerateCategories(nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnumerateCategories(_retval); } \


#endif /* __gen_nsICategoryManager_h__ */
