use affine_core::access_control::{AccessGrant, Limits, Plan};

pub(crate) const SELFHOST_FREE_MEMBER_LIMIT: i32 = 10_000;

pub(crate) fn limits_for_plan(plan: &Plan, mut limits: Limits) -> Limits {
  // affine_core supplies a 10-seat default for this plan.
  if matches!(plan, Plan::SelfHostedFree) {
    limits.seat_limit = SELFHOST_FREE_MEMBER_LIMIT;
  }
  limits
}

pub(crate) fn grant_with_member_limit(mut grant: AccessGrant) -> AccessGrant {
  grant.limits = limits_for_plan(&grant.plan, grant.limits);
  grant
}

#[cfg(test)]
mod tests {
  use affine_core::access_control::{
    QuotaUsage, ReadonlyReason, SeatDenialReason, SeatOperationPlan, SeatReservationKind, SeatReviewFacts, SeatReviewRequest,
    describe_plan, evaluate_workspace_quota, plan_seat_review_reservation,
  };

  use super::*;

  #[test]
  fn selfhost_free_has_ten_thousand_seats() {
    let grant = grant_with_member_limit(describe_plan(Plan::SelfHostedFree, None).unwrap().into());
    assert_eq!(grant.limits.seat_limit, 10_000);

    for (charged_seats, overflow) in [(9_999, false), (10_000, false), (10_001, true)] {
      let state = evaluate_workspace_quota(
        &grant,
        QuotaUsage {
          storage_bytes: 0,
          charged_seats,
        },
      );
      assert_eq!(state.readonly_reasons.contains(&ReadonlyReason::MemberOverflow), overflow);
    }

    for (charged_seats, allowed) in [(9_999, true), (10_000, false)] {
      let decision = plan_seat_review_reservation(
        SeatReviewRequest {
          reservation: SeatReservationKind::EmailInvite,
          requested_seats: 1,
          collision: false,
          authority_allowed: true,
        },
        SeatReviewFacts {
          grant: grant.clone(),
          usage: QuotaUsage {
            storage_bytes: 0,
            charged_seats,
          },
        },
      );
      assert_eq!(matches!(decision, SeatOperationPlan::Mutate(_)), allowed);
      if !allowed {
        assert_eq!(
          decision,
          SeatOperationPlan::Deny(SeatDenialReason::SeatLimitExceeded { limit: 10_000 })
        );
      }
    }
  }

  #[test]
  fn paid_selfhost_and_cloud_limits_stay_unchanged() {
    for (plan, quantity) in [(Plan::SelfHostedTeam, Some(5)), (Plan::Free, None)] {
      let grant: AccessGrant = describe_plan(plan, quantity).unwrap().into();
      let original_limit = grant.limits.seat_limit;
      let adjusted = grant_with_member_limit(grant);
      assert_eq!(adjusted.limits.seat_limit, original_limit);
    }
  }
}
