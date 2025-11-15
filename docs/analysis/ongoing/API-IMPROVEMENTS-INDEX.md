# API Improvements - Action Items Index

**Last Updated:** 2024-11-15
**Status:** Planning Phase
**Target Release:** v0.2.0

This directory contains detailed implementation plans for architecture and API improvements to rs-pfcp, based on a comprehensive code review conducted in November 2024.

## Overview

The rs-pfcp library has excellent foundations with clean architecture and strong 3GPP compliance. These action items will elevate it from "very good" to "exceptional" by improving:
- API stability and future-proofing
- Type safety
- Developer ergonomics
- Error handling

## Action Items by Priority

### 🔴 High Priority (Must Have for v0.2.0)

These items significantly improve API stability and developer experience:

1. **[Private Fields Encapsulation](./private-fields-encapsulation.md)**
   - **Issue:** Public struct fields expose implementation details
   - **Impact:** API instability, no future-proofing
   - **Effort:** Medium (2-3 days)
   - **Breaking:** Yes
   - **Status:** 📋 Planned

2. **[Custom Error Type (PfcpError)](./custom-error-type.md)**
   - **Issue:** All errors are generic `io::Error`
   - **Impact:** Poor debugging, no error recovery
   - **Effort:** Medium (3-4 days)
   - **Breaking:** Yes
   - **Status:** 📋 Planned

3. **[API Stability Guarantees](./api-stability-guarantees.md)**
   - **Issue:** No documented stability policy
   - **Impact:** User uncertainty about API changes
   - **Effort:** Low (1-2 days)
   - **Breaking:** No
   - **Status:** ✅ Completed

### 🟡 Medium Priority (Should Have for v0.2.0)

These items improve consistency and prevent common bugs:

4. **[Unified IE Access Patterns](./unified-ie-access.md)**
   - **Issue:** Inconsistent IE access (`find_ie` vs `find_all_ies`)
   - **Impact:** User confusion, "first only" bugs
   - **Effort:** Medium (2-3 days)
   - **Breaking:** No (with deprecation)
   - **Status:** 📋 Planned

5. **[Newtype Wrappers](./newtype-wrappers.md)**
   - **Issue:** Primitive types in constructors (easy to swap)
   - **Impact:** Runtime bugs from swapped arguments
   - **Effort:** Low (1-2 days)
   - **Breaking:** Yes (minor)
   - **Status:** 📋 Planned

6. **[Expand IntoIe Trait](./expand-into-ie-trait.md)**
   - **Issue:** Limited ergonomic conversions
   - **Impact:** Verbose builder usage
   - **Effort:** Low (1 day)
   - **Breaking:** No
   - **Status:** 📋 Planned

### 🟢 Low Priority (Nice to Have)

These items provide polish and convenience:

7. **[Default Trait Implementations](./default-trait-implementations.md)**
   - **Issue:** Builders lack `Default` trait
   - **Impact:** Can't use Rust idioms
   - **Effort:** Low (0.5 day)
   - **Breaking:** No
   - **Status:** 📋 Planned

8. **[Marshal Into Buffer Variants](./marshal-into-variants.md)**
   - **Issue:** Marshaling always allocates
   - **Impact:** Performance in hot paths
   - **Effort:** Low (1 day)
   - **Breaking:** No
   - **Status:** 📋 Planned

9. **[Builder Documentation](./builder-documentation.md)**
   - **Issue:** No unified builder guide
   - **Impact:** Harder onboarding
   - **Effort:** Low (1 day)
   - **Breaking:** No
   - **Status:** 📋 Planned

## Implementation Timeline

### Phase 1: Breaking Changes (Week 1-2)
Focus on items that require API breaks (do these together):
- ✅ Action #1: Private Fields Encapsulation
- ✅ Action #2: Custom Error Type
- ✅ Action #5: Newtype Wrappers

### Phase 2: API Enhancements (Week 2-3)
Add new features without breaking existing code:
- ✅ Action #4: Unified IE Access
- ✅ Action #6: Expand IntoIe Trait
- ✅ Action #7: Default Traits

### Phase 3: Documentation & Polish (Week 3-4)
Document everything and add performance optimizations:
- ✅ Action #3: API Stability Guarantees (Completed)
- ✅ Action #8: Marshal Into Variants (Completed)
- ✅ Action #9: Builder Documentation

### Phase 4: Testing & Release (Week 4)
- Full test suite validation
- Update all examples
- Migration guide completion
- v0.2.0 release

## Estimated Total Effort

- **High Priority:** 6-9 days
- **Medium Priority:** 4-6 days
- **Low Priority:** 2.5 days
- **Total:** 12.5-17.5 days (2.5-3.5 weeks)

## Dependencies Between Items

```
Private Fields (#1) ─┐
                     ├─→ Unified IE Access (#4)
Custom Error (#2) ───┘

Newtype Wrappers (#5) ─→ Builder Docs (#9)

Expand IntoIe (#6) ───→ Builder Docs (#9)
```

**Recommendation:** Implement in numerical order within each priority group.

## Success Metrics

After implementing all items, the library should have:

- [ ] **Stable Public API** - Private fields, versioned guarantees
- [ ] **Better Error Messages** - Structured errors with context
- [ ] **Type Safety** - Newtype wrappers prevent bugs
- [ ] **Consistent Patterns** - Unified IE access
- [ ] **Good Documentation** - API guides and examples
- [ ] **Performance** - No regressions, optional optimizations
- [ ] **User Satisfaction** - Easier to use, fewer footguns

## Review Checklist

Before implementing each action item:

- [ ] Read the detailed document
- [ ] Review code examples
- [ ] Understand breaking changes
- [ ] Check dependencies on other items
- [ ] Confirm timeline fits overall plan

## Getting Started

1. **Review this index** to understand all items
2. **Read high-priority documents** first
3. **Create feature branch** for v0.2.0 work
4. **Implement in phases** as outlined above
5. **Test thoroughly** after each phase
6. **Update documentation** continuously

## Questions or Feedback?

- Create a GitHub issue for discussion
- Reference the specific action item document
- Tag with `api-improvement` label

## Version History

- **2024-11-15:** Initial action items created
- **Future:** Track implementation progress here

---

**Note:** These documents are living specifications. Update them as implementation progresses and requirements evolve.
