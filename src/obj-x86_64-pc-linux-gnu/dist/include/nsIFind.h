/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/find/nsIFind.idl
 */

#ifndef __gen_nsIFind_h__
#define __gen_nsIFind_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIWordBreaker; /* forward declaration */

class nsRange; /* webidl Range */


/* starting interface:    nsIFind */
#define NS_IFIND_IID_STR "40aba110-2a56-4678-be90-e2c17a9ae7d7"

#define NS_IFIND_IID \
  {0x40aba110, 0x2a56, 0x4678, \
    { 0xbe, 0x90, 0xe2, 0xc1, 0x7a, 0x9a, 0xe7, 0xd7 }}

class NS_NO_VTABLE nsIFind : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFIND_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFind;

  /* attribute boolean findBackwards; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFindBackwards(bool *aFindBackwards) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFindBackwards(bool aFindBackwards) = 0;

  /* attribute boolean caseSensitive; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCaseSensitive(bool *aCaseSensitive) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCaseSensitive(bool aCaseSensitive) = 0;

  /* attribute boolean entireWord; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEntireWord(bool *aEntireWord) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEntireWord(bool aEntireWord) = 0;

  /* attribute boolean matchDiacritics; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) = 0;

  /* Range Find (in AString aPatText, in Range aSearchRange, in Range aStartPoint, in Range aEndPoint); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Find(const nsAString& aPatText, nsRange *aSearchRange, nsRange *aStartPoint, nsRange *aEndPoint, nsRange **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFind, NS_IFIND_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFIND \
  NS_IMETHOD GetFindBackwards(bool *aFindBackwards) override; \
  NS_IMETHOD SetFindBackwards(bool aFindBackwards) override; \
  NS_IMETHOD GetCaseSensitive(bool *aCaseSensitive) override; \
  NS_IMETHOD SetCaseSensitive(bool aCaseSensitive) override; \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override; \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override; \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override; \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override; \
  NS_IMETHOD Find(const nsAString& aPatText, nsRange *aSearchRange, nsRange *aStartPoint, nsRange *aEndPoint, nsRange **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFIND \
  nsresult GetFindBackwards(bool *aFindBackwards); \
  nsresult SetFindBackwards(bool aFindBackwards); \
  nsresult GetCaseSensitive(bool *aCaseSensitive); \
  nsresult SetCaseSensitive(bool aCaseSensitive); \
  nsresult GetEntireWord(bool *aEntireWord); \
  nsresult SetEntireWord(bool aEntireWord); \
  nsresult GetMatchDiacritics(bool *aMatchDiacritics); \
  nsresult SetMatchDiacritics(bool aMatchDiacritics); \
  nsresult Find(const nsAString& aPatText, nsRange *aSearchRange, nsRange *aStartPoint, nsRange *aEndPoint, nsRange **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFIND(_to) \
  NS_IMETHOD GetFindBackwards(bool *aFindBackwards) override { return _to GetFindBackwards(aFindBackwards); } \
  NS_IMETHOD SetFindBackwards(bool aFindBackwards) override { return _to SetFindBackwards(aFindBackwards); } \
  NS_IMETHOD GetCaseSensitive(bool *aCaseSensitive) override { return _to GetCaseSensitive(aCaseSensitive); } \
  NS_IMETHOD SetCaseSensitive(bool aCaseSensitive) override { return _to SetCaseSensitive(aCaseSensitive); } \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override { return _to GetEntireWord(aEntireWord); } \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override { return _to SetEntireWord(aEntireWord); } \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override { return _to GetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override { return _to SetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD Find(const nsAString& aPatText, nsRange *aSearchRange, nsRange *aStartPoint, nsRange *aEndPoint, nsRange **_retval) override { return _to Find(aPatText, aSearchRange, aStartPoint, aEndPoint, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFIND(_to) \
  NS_IMETHOD GetFindBackwards(bool *aFindBackwards) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFindBackwards(aFindBackwards); } \
  NS_IMETHOD SetFindBackwards(bool aFindBackwards) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFindBackwards(aFindBackwards); } \
  NS_IMETHOD GetCaseSensitive(bool *aCaseSensitive) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCaseSensitive(aCaseSensitive); } \
  NS_IMETHOD SetCaseSensitive(bool aCaseSensitive) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCaseSensitive(aCaseSensitive); } \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntireWord(aEntireWord); } \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEntireWord(aEntireWord); } \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD Find(const nsAString& aPatText, nsRange *aSearchRange, nsRange *aStartPoint, nsRange *aEndPoint, nsRange **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Find(aPatText, aSearchRange, aStartPoint, aEndPoint, _retval); } 


#endif /* __gen_nsIFind_h__ */
