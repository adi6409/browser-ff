/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* Generated with cbindgen:0.16.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen. See RunCbindgen.py */
#ifndef mozilla_intl_l10n_FluentBindings_h
#error "Don't include this file directly, instead include FluentBindings.h"
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace mozilla {
namespace intl {
namespace ffi {

enum class FluentDateTimeStyle {
  Full,
  Long,
  Medium,
  Short,
  None,
};

enum class FluentNumberCurrencyDisplayStyleRaw {
  Symbol,
  Code,
  Name,
};

enum class FluentNumberStyleRaw {
  Decimal,
  Currency,
  Percent,
};

/// Base class for a [`FluentBundle`] struct. See its docs for details.
/// It also is implemented for [`concurrent::FluentBundle`].
///
/// [`FluentBundle`]: ../type.FluentBundle.html
/// [`concurrent::FluentBundle`]: ../concurrent/type.FluentBundle.html
template<typename R = void, typename M = void>
struct FluentBundleBase;

/// A resource containing a list of localization messages.
struct FluentResource;

struct IntlLangMemoizer;

struct RawDateTimeFormatter;

struct RawNumberFormatter;

template<typename T = void>
struct Rc;

/// A collection of localization messages for a single locale, which are meant
/// to be used together in a single view, widget or any other UI abstraction.
///
/// # Examples
///
/// ```
/// use fluent_bundle::{FluentBundle, FluentResource, FluentValue, FluentArgs};
/// use unic_langid::langid;
///
/// let ftl_string = String::from("intro = Welcome, { $name }.");
/// let resource = FluentResource::try_new(ftl_string)
///     .expect("Could not parse an FTL string.");
///
/// let langid_en = langid!("en-US");
/// let mut bundle = FluentBundle::new(vec![langid_en]);
///
/// bundle.add_resource(&resource)
///     .expect("Failed to add FTL resources to the bundle.");
///
/// let mut args = FluentArgs::new();
/// args.add("name", FluentValue::from("Rustacean"));
///
/// let msg = bundle.get_message("intro").expect("Message doesn't exist.");
/// let mut errors = vec![];
/// let pattern = msg.value.expect("Message has no value.");
/// let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);
/// assert_eq!(&value, "Welcome, \u{2068}Rustacean\u{2069}.");
///
/// ```
///
/// # `FluentBundle` Life Cycle
///
/// ## Create a bundle
///
/// To create a bundle, call [`FluentBundle::new`] with a locale list that represents the best
/// possible fallback chain for a given locale. The simplest case is a one-locale list.
///
/// Fluent uses [`LanguageIdentifier`] which can be created using `langid!` macro.
///
/// ## Add Resources
///
/// Next, call [`add_resource`] one or more times, supplying translations in the FTL syntax.
///
/// Since [`FluentBundle`] is generic over anything that can borrow a [`FluentResource`],
/// one can use [`FluentBundle`] to own its resources, store references to them,
/// or even [`Rc<FluentResource>`] or [`Arc<FluentResource>`].
///
/// The [`FluentBundle`] instance is now ready to be used for localization.
///
/// ## Format
///
/// To format a translation, call [`get_message`] to retrieve a [`FluentMessage`],
/// and then call [`format_pattern`] on the message value or attribute in order to
/// retrieve the translated string.
///
/// The result of [`format_pattern`] is an [`Cow<str>`]. It is
/// recommended to treat the result as opaque from the perspective of the program and use it only
/// to display localized messages. Do not examine it or alter in any way before displaying.  This
/// is a general good practice as far as all internationalization operations are concerned.
///
/// If errors were encountered during formatting, they will be
/// accumulated in the [`Vec<FluentError>`] passed as the third argument.
///
/// While they are not fatal, they usually indicate problems with the translation,
/// and should be logged or reported in a way that allows the developer to notice
/// and fix them.
///
///
/// # Locale Fallback Chain
///
/// [`FluentBundle`] stores messages in a single locale, but keeps a locale fallback chain for the
/// purpose of language negotiation with i18n formatters. For instance, if date and time formatting
/// are not available in the first locale, [`FluentBundle`] will use its `locales` fallback chain
/// to negotiate a sensible fallback for date and time formatting.
///
/// # Concurrency
///
/// As you may have noticed, `FluentBundle` is a specialization of [`FluentBundleBase`]
/// which works with an [`IntlMemoizer`][] over `RefCell`.
/// In scenarios where the memoizer must work concurrently, there's an implementation of
/// `IntlMemoizer` that uses `Mutex` and there's [`concurrent::FluentBundle`] which works with that.
///
/// [`add_resource`]: ./bundle/struct.FluentBundleBase.html#method.add_resource
/// [`FluentBundle::new`]: ./bundle/struct.FluentBundleBase.html#method.new
/// [`FluentMessage`]: ./struct.FluentMessage.html
/// [`FluentBundle`]: ./type.FluentBundle.html
/// [`FluentResource`]: ./struct.FluentResource.html
/// [`get_message`]: ./bundle/struct.FluentBundleBase.html#method.get_message
/// [`format_pattern`]: ./bundle/struct.FluentBundleBase.html#method.format_pattern
/// [`Cow<str>`]: http://doc.rust-lang.org/std/borrow/enum.Cow.html
/// [`Rc<FluentResource>`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
/// [`Arc<FluentResource>`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
/// [`LanguageIdentifier`]: https://crates.io/crates/unic-langid
/// [`IntlMemoizer`]: https://github.com/projectfluent/fluent-rs/tree/master/intl-memoizer
/// [`Vec<FluentError>`]: ./enum.FluentError.html
/// [`concurrent::FluentBundle`]: ./concurrent/type.FluentBundle.html
template<typename R>
using FluentBundle = FluentBundleBase<R, IntlLangMemoizer>;

using FluentBundleRc = FluentBundle<Rc<FluentResource>>;

struct FluentArgument {
  enum class Tag : uint8_t {
    Double_,
    String,
  };

  struct Double__Body {
    double _0;
  };

  struct String_Body {
    const nsCString *_0;
  };

  Tag tag;
  union {
    Double__Body double_;
    String_Body string;
  };

  static FluentArgument Double_(const double &_0) {
    FluentArgument result;
    ::new (&result.double_._0) (double)(_0);
    result.tag = Tag::Double_;
    return result;
  }

  bool IsDouble_() const {
    return tag == Tag::Double_;
  }

  static FluentArgument String(const nsCString *const &_0) {
    FluentArgument result;
    ::new (&result.string._0) (const nsCString*)(_0);
    result.tag = Tag::String;
    return result;
  }

  bool IsString() const {
    return tag == Tag::String;
  }
};

struct FluentNumberOptionsRaw {
  FluentNumberStyleRaw style;
  nsCString currency;
  FluentNumberCurrencyDisplayStyleRaw currency_display;
  bool use_grouping;
  uintptr_t minimum_integer_digits;
  uintptr_t minimum_fraction_digits;
  uintptr_t maximum_fraction_digits;
  intptr_t minimum_significant_digits;
  intptr_t maximum_significant_digits;
};

struct FluentDateTimeOptionsRaw {
  FluentDateTimeStyle date_style;
  FluentDateTimeStyle time_style;
  nsCString skeleton;
};

extern "C" {

FluentBundleRc *fluent_bundle_new_single(const nsACString *locale,
                                         bool use_isolating,
                                         const nsACString *pseudo_strategy);

FluentBundleRc *fluent_bundle_new(const nsCString *locales,
                                  uintptr_t locale_count,
                                  bool use_isolating,
                                  const nsACString *pseudo_strategy);

void fluent_bundle_get_locales(const FluentBundleRc *bundle, nsTArray<nsCString> *result);

void fluent_bundle_destroy(FluentBundleRc *bundle);

bool fluent_bundle_has_message(const FluentBundleRc *bundle, const nsACString *id);

bool fluent_bundle_get_message(const FluentBundleRc *bundle,
                               const nsACString *id,
                               bool *has_value,
                               nsTArray<nsCString> *attrs);

bool fluent_bundle_format_pattern(const FluentBundleRc *bundle,
                                  const nsACString *id,
                                  const nsACString *attr,
                                  const nsTArray<nsCString> *arg_ids,
                                  const nsTArray<FluentArgument> *arg_vals,
                                  nsACString *ret_val,
                                  nsTArray<nsCString> *ret_errors);

void fluent_bundle_add_resource(FluentBundleRc *bundle,
                                const FluentResource *r,
                                bool allow_overrides,
                                nsTArray<nsCString> *ret_errors);

extern RawNumberFormatter *FluentBuiltInNumberFormatterCreate(const nsCString *locale,
                                                              const FluentNumberOptionsRaw *options);

extern uint8_t *FluentBuiltInNumberFormatterFormat(const RawNumberFormatter *formatter,
                                                   double input,
                                                   uint32_t *out_count);

extern void FluentBuiltInNumberFormatterDestroy(RawNumberFormatter *formatter);

extern RawDateTimeFormatter *FluentBuiltInDateTimeFormatterCreate(const nsCString *locale,
                                                                  const FluentDateTimeOptionsRaw *options);

extern uint8_t *FluentBuiltInDateTimeFormatterFormat(const RawDateTimeFormatter *formatter,
                                                     double input,
                                                     uint32_t *out_count);

extern void FluentBuiltInDateTimeFormatterDestroy(RawDateTimeFormatter *formatter);

const FluentResource *fluent_resource_new(const nsACString *name, bool *has_errors);

void fluent_resource_addref(const FluentResource *res);

void fluent_resource_release(const FluentResource *res);

} // extern "C"

} // namespace ffi
} // namespace intl
} // namespace mozilla
