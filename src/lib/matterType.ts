import type { CaseProfile } from './types/case';

export function isLitigationMatter(profile: CaseProfile | null) {
  if (!profile) return false;
  if (['transactional', 'administrative'].includes(profile.matter_tier || '')) return false;
  return !['document', 'will', 'lease', 'contractor', 'power_of_attorney', 'bill_of_sale', 'nda', 'demand_letter', 'cure_notice', 'security_deposit_demand', 'records_request', 'agency_appeal'].includes(profile.document_kind);
}
