pub mod solver {
    use std::collections::{HashMap, HashSet};

    pub fn solve_task1(input: &str) -> u32 {
        let overlaps = collect_operlaps(input);
        return calculate_priority_sum(&overlaps);
    }

    pub fn solve_task2(input: &str) -> u32 {
        let badge = collect_badges(input);
        return calculate_priority_sum(&badge);
    }

    fn collect_operlaps(input: &str) -> Vec<char> {
        let mut overlaps  = Vec::new();
        for line in input.lines() {
            overlaps.extend(line_overlaps(line));
        }

        return overlaps;
    }

    fn line_overlaps(line: &str) -> Vec<char> {
        let (c1,c2) = line.split_at(line.len()/2);

        let overlaps  = find_overlaps(c1, c2);

        return overlaps.iter().map(|k| *k).collect();
    }

    fn find_overlaps(c1: &str,c2: &str) -> HashSet<char> {
        let c1map = c1.chars()
            .map(|c| (c as u8, true))
            .collect::<HashMap<u8, bool>>();

        let mut overlaps  = HashSet::new();
        for c in c2.chars() {
            if c1map.contains_key(&(c as u8)) {
                overlaps.insert(c);
            }
        }
        return overlaps;
    }

    fn collect_badges(input: &str) -> Vec<char> {
        let mut badges = Vec::new();
        // process input  3 lines at a time
        let lines = input.lines().collect::<Vec<&str>>();
        lines.chunks(3).for_each(|chunk| {
            let (l1,l2,l3) = (chunk[0], chunk[1], chunk[2]);
            let o1 = find_overlaps(l1, l2);
            let o2 = find_overlaps(l2, l3);
            let o3 = find_overlaps(l1, l3);

            // find the intersection of the 3 sets
            let o4 = o1.intersection(&o2).map(|c| *c).collect::<HashSet<char>>();
            let badge = o4.intersection(&o3).map(|c| *c).collect::<Vec<char>>();

            badges.push(badge[0]);
        });

        return badges;
    }

    fn calculate_priority_sum(overlaps: &Vec<char>) -> u32 {
        let mut sum = 0;
        for c in overlaps {
            let p = priority(c) as u32;
            sum += p;
        }
        return sum;
    }

    fn priority(c: &char) -> u8 {
        let b = *c as u8;
        match b {
            65..=90 => b-64+26, // A-Z
            97..=122 => b-96, // a-z
            _ => 0, // not a letter
        }
    }
}

// test task1 solver
#[cfg(test)]
mod tests {
    use super::solver;

    #[test]
    fn test_task1_example() {
        assert_eq!(solver::solve_task1(EXAMPLE_INPUT), 157);
    }

    #[test]
    fn test_task1_real() {
        assert_eq!(solver::solve_task1(REAL_INPUT), 8185);
    }

    #[test]
    fn test_task2_example() {
            assert_eq!(solver::solve_task2(EXAMPLE_INPUT), 70);
    }

    static EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    static REAL_INPUT: &str = "|
shzsFcPssFhjFssBzdpRcNHNZrpdJdJVJZ
fwvMCntfCCbSbSbtDgDNrDtDtJHZVH
GbCwwbwwnGrLhBzjFFFsWPhL
PpCqRsqqmmtCwMJC
LHFrLLHDSNHlfWNhDzmjzzJlJzPJMvPJjQ
SGSWDNrhZhPDSWDZLgVVRgbRppgpGVnpnn
GRRjbVjmJZlgMRzzrN
FpDptHpfHfnpPTvDFTWpFPnPcMfNCClNrzcVcrMMzVsCZlsZ
TFTQDnvLHPFDtVbLwbjdGjdwwJ
lhljvvhCjjzhjszzBPmnmGVZMGzG
FbTcTwbtSFdtcMPnTBPQVnnBZT
SFMpHDtNDSSbSdwppvgJWjJCJJgWgvlJHH
wzNCWpzCzJnWWpRRNdJrgHLhjfbLrHrchV
lBMStmPmmLQDPQZlshrdhgrfrcrrddgHgs
mvGDGQSvDPBlGMLGCvCWpNvpzRWFwqRw
stBttBThtDZqPWssPWZp
gRggwwggCGFSBBvPRpHZZrHdZLZq
ccFJGCNJmmGQzbTDhnQhBBnB
HJqMqtZbJMmJTqtLtVMqhpfphNdQfhfzzjhhlHll
rWSBrnwFwWCvwWCwBgPgCgzjQccQhhgRzcdQzjfcNfzR
CWBCwCvCvvwssWLMtJJGMdMZJsGV
nFwSFQwsNrrsssSwCrhrCNnfcCRgJRMJTJcMfRzMCMCRvW
DdbGdLZLttllWWvTzgzzgR
ZqGzPdLtDjBjDZGPZVmnhQFwqrFQhVFnss
sNNpCjttjsJjSpgpWjslCTnqqSVffrnhSfDhmhrhfm
dBwcGzbPBHbbwZcwJbcTTFDFFFDVrdVmFdnDqf
HzGcczQPHGwzPzGHRctWlvRgtvJlvNlJvRNj
cFNCFdvcCHvFBCZcwBfRSpttGhDmCghGShmSRt
QjLnTTzQVzTTnLMqhDgPhGDDSjGPrgSh
TTJGnJJlLQdNWZWJNBJZ
WHBpHcMDZHLDbHLtGCnmRmLNGmRqvsCC
PzTFzPPTJzrSbGsvnmqfsqfqRz
dJSQQdVFQgjTrjQPWcWHbBVcZVccZtWp
JDtnRtJzNzTTNlHc
rQPJFrLPGMMwrGPFwjFMVLjSTWHdWBTdSWdWZlcWTHlZ
MGFrCvLLwrwPFVVhvLMGGtnqfsmRJgDnqbRgfbqmfC
jnTtFjcSSvctJjznzvFmpqqPMqQDRVpRqPzqQzVQ
bhHBfrWpfHsfGNllRrRCPqDCqPqq
gsGZZpbWgbwHWGNgfZNjvLSTTjtnTgjSSSSJmL
RLQNdVNnRQdQHVVLGpspNqvtsqptqpTtsp
MlRWwbRBBFMFjCTFTTFDvj
WmlWBmBwwmrndnmLRHRQ
WnftJWlfnWSHGCjWWWSCFqFGBDqBwMcDmmMmGmqD
pTNhpTrPhhhRPzbhrppLhThLgqDmwccwqPBmMMqnFBcwwmBB
ZrhQTpzdjSVVtnvZ
jgtnJtBjtlTdJBZJVQBngQGDCGWpPGCcPWCbWdWMbcpP
wHstNNttSHPDmHcMCp
rrFFSvLLNfsFtNSqrtfrhsNjjVTBVgVvnTBvTQvjTgjVZz
qhZwlqFqFwlJwrDHqHcDvgcNzv
RCCTQmjCbQTBtRTbjJRDpBrgDHPPpPDvHccDPc
VmjVWstQJhlJlGVJ
GggpGwZmgvgJMvbJFQQDbDFbBbFCQDCW
rtrLzNLtNSPnNqDSQDcQCWlqBQ
VtzdRPtztLtVRtZmmMTRwCGZpMwp
vtvqjsCqtshfjcWFHWGjGFJj
zGrnzDDMpPcTHcSTVTJP
DbDwMbZRDrZdBBnMznZMGZDfwtlgQhsqCttClsqvsLfCff
JLzLtLsrzsQdvrWRwMHwcc
qPmCTzlPjljjFTZmWwcwwvHMMRWwHvMm
PFqZnVCqTCNjCzNgQsbGBLzLQQ
CBnppDHllVpPCBshBHpjDTSmZcSrfwvmJcDDTJfw
dFRLdLFQzNSTBTSNmBJv
FzFFzRMBFWtQlPlsjjPVMnhC
CVCfwnfdVvBdBbTNTT
LNzsHPNWsDjTZqDHqT
PtLgQsGQLSzWLstPgGWcgQLSNrpplffrnrNhpVCwlGVlrwMn
jPPVqPsHffzVnHzvSgMcCJGGMSVCll
pdbpDpBLNmNNppJgcvgSllGjDSGQ
hrbBwLpjLhhhNZLhNrhZZLHzfsztFzzsrtHfRFnFfRHf
tdjBdbmSfdHBdHHmZlWjFrnlWQlqvMFvFn
pDNDJhLhPVPLLLJphJLwNcwnQTcWWTqTrqMWTZvqMrlvFM
gpVLhNwpgZJCghCLDNwphgmsBdzHHHmSstmfggzdbR
TfMpfMBVftLMDBSjWDHgzHbgwLgHHvdzggzs
QJnZcFFnZRHdHjJvwgdg
RjjRRnmNmmZNjZqZnQcVffBrWqVTqrtffTSTVV
fZTdTVcVjrjdBzdTnGtgnnGSHHNFGn
thMWPtPMslmGnWnNnS
thvbMvQMRphhLCjrzBjZVdcQfC
MpmgZFgMGdrFrBCVnJ
JsbJlTTlvLQbVffRRvBBRVjd
LWlbhHlJhLTJmmGcMMHNmNgN
bhvmhPrbhqNqQRRGzQjVvvRL
wTwBZDBTwwggfnngcDfdsVVFQCdzCzDVRsFdQs
pngWMcgzMgpZWncnMpWNrbNHrNbmHhltWlbl
nPndBjLPscWSccBVGnScsSzMdhMppMthdMgpMgrzvhhp
CCFTFDwqZqCCJmhvpDzztVzDNztp
qQFJTbRVbmCfwTwfmnnssWBGnLnWlRLSGn
JRlJDSvLRRCdvmDSvdlbZNVBSWZGNgWsZGNgZBVs
QrjPMqMnLzzjLjFnNNgBpsgtgGGGVZ
hrjrFqjqFrQfMHPhQzDvvCLJdwwwmvbJbwDH
HDGrDDDpNsGQNdZQ
jpjgtgjSjpjllfZZtZsvNdtshqqq
cbgMfjclWTJcMwjWJfpfmVPLPBnVBHnmVbnmLBbD
rPrMZNsNrsvrwqvFFFdgQWNzLJJzRW
pStppStHmcmHpgVSllVcbVbWWDdLFhdbzdRRFhJFLLRF
cltCHmCBmtSlgjpllgGvTwPZPMfZvPsCMCwZvC
FRQQMdlFMDWRFQRQMQQDWdFbSSSVJSBbJSlBVVBnPJnzJL
rsftPfhsrgwznSzzHSLgJG
fhNsjrjhvsTTvdjcCRMRMRPcCW
tRtJttHFrjtDQHHBQMMBgMBSghhZQb
vqWPLpLvqrmPdmqwvqfmPhNBBBlSnbwbgnlnlhNSZZ
pGpdfzLLspddmqsqPvfvvPpGTVcJJCDRjHrccRtDjcRDFD
GJMHCdTMWJRhSTlhhSPllt
fVvqpfBFrqvqNzzgVDFrpDPmSVtQSlSmhjwltlRtmVhn
pzpBNDBzfDrsNsDRJJRdCssMLdLZWZ
hFfvWWvdpCwwcwFhphpcZCMmllHLfmbQlbrQLBJmGgQrQm
nVSNGjGzzSVNTsjzSJrbSrHBSHlrrmQHJB
PttTNsTRVnNjNqRnzRzRWCCCcpMCWGPMwwFZvFwW
DvZbFnDDsqDBwwRQgNBm
HhWpWWRMWChlChdHLlGlGtQtggSNPSSpgPmNgPJQtw
CMlWGMhhCVHlLCdHTHdrGHdbzjVqnzcvqqjRjFbbzbFFqR
ZZgCNqqBmjZsNgZCqJgNBdrLFHbBrWlPdHWFbPnHPW
TVwTDfzDwSDzmcSTcrzdbllnHPHdFlLzbF
tvDQwVtVvDVmtRsNMgRpJg
BBpDCpNJnmnpnDDmDGGmtTzqHcGTvTTjTbGjHLVcLb
swNNhPwwHzTVwwHw
rPRlPRhSQmmBDpnNfl
pbRhffPzcPDmfcNTpVBLpBjMGBGjZLLg
ssrCsqrszgJjZMqZLQ
SzCwnsllCrssvdrvwzPmDmPPbFRbSThPTPDD
QWLfcfczQQpcDTpLPfdZRRvRRVqbFWvZbvtqvv
NsGGJBhCmNdZVqsbdssZ
rMwwwBBJMrdzPfQMpzMnLQ
rdtCQhrCtQQprtTWQCHFjPgGBPdFPgvBqRRPqB
lsVsSnVSbLmmgBcgTLTFGgvq
wTTDTszsbzMDppJrJhDQ
ZlmsGLBVCBBZFCFFHqcHVvQhqVQSSHpH
dbbTRMrRwwzDfrTbFtMvcptvHFQQpqtc
gdJTDWgfwDwTwmgmNPnNsgFBlZ
PWhWhGFzzzrLdHCPccbJQJcHPD
NRpVTpTgRWVlHJNHMcHQMb
pSRSpVSZWRSTZjgTRTWnFLdZLrrndhdzZvtvzn
LgctLgVBVLhlPjqRhBLVcVlhbDDcGnNGfwCrbNDnrGbGCJNw
HmppHMWWmQmMqZZHWQrDDfDCffJrDGJrCb
qpWZsZZZMZWTLPhTRgTtThRP
hfhQfFQWzBfhfTQdmzdLDtDjtvHLjt
qsgpcqMNRmgpqsCwpCmZDvjwvdddHZHvZDtrrd
SgmNmqScbTSJbhJQ
dvMTQvTnZJsrQdbbSvMVZMblDwlflfDGgwwHcfGjPfjjrG
FqBLpBpFpFzRzqNFmgjGlDRHcwPPGwgcgs
tLNtWshsLLqWMJhTQVVbhvJd
bgZLMZgzbbLCcPCbMZbcNMgBqSTqSWVtSzvvTTBTqBvRBW
FhQpJQnGrlhGlrnTqRtTRJqSwDwtVR
FqlnnFnqFHnGHHdNZdbZbCNMdPLMPb
HHFnbftcfnfbbTbTnHTNVZZzJlPQlFrFzVJFZsdr
mvpGCBgwqCvLCqvMQWWzsQQWlPzwzsrV
hpGSGgqSvqbHcnhfVfct
lGVrnHsGcnVHzscrlGjHcrHqqWPlJCPJClTLLqCSPLPdqS
fRbwbtMQZtMMRFMSqfJTTWCTJPJCmd
ggdvtvdbVVGnpDGg
BnBjTcbnvhjjlMnNJJfnDnQDGdNDfP
qwFqVSWwqLpWFmFVCSqFpDDCJNJRQTRfRDGPfdfDQN
zwHwWVVWWFSqqwWTLqzzztHMvBhlcghblMlcttMllh
PFFNPNPmlFllbctNLmcjBstrsVrQHJSSHHSnnB
fddfDhdwGhTWWTDMwMggssjsjndsBsjsnSrVqSVV
MCvvTWvRMwCvGPpzCcmbplpCVC
thTqlPPTNbGNhGdqRRhRrNtFWnDnvvFZDpnFvfQDZtvWvv
HcMzVcVVcHrgHzcMcmmgfQvFQnMjnWDjfnvjQFfQ
VSmHJLHBJrTrJTlT
NjnsHjLLjNRddNdBFBSR
ftsbqfDcDqsrDtqsfSVBhJVFJgdBRVFS
wvDqwtDlsDDczjzjHvLzLQQM
qDwstwDtRfpJfVhBVZBMvnlRvv
zSFzQHFWdgZBVTZhTzrp
NHdggjGjWHQFPWHNPPbpJfPDtCwCtqDqJfbt
pvnbqHvnTvlCCpjsBsMGBGWWPp
RJSJhJCRVJmJwScrhSJdfwFsBGhZBjhGFFFFGgFPGhZW
cdRrdmwtfcdSmLtcSCQlvQNqQTlqqvtTlv
rnSlSrgWjVGpTTRhSffpRd
HtgHPsNNgNHszPcTBphMdhHhBcTc
JNNbZPZZsszNmtDbPgsmJlwFvWVnCwrlmWlnjnGvCC
WrVBVgVGGQCrSTTqvVjDqDjv
FmwRRwwRQhhLFMjFMzdqSSzS
RcJtbnnLtQWrGHcfrP
vpzssjmVjVZWNZzzQwtQwccpQhgtQCct
qDdfLMnMrrTbBLqTqltlTfbGQnghgRwggGgRnhhccCJJcG
MtdLfSSSddMftlrjjzsSWVSjFvjNvs
qTRPpRPzJglzGJzpGRHWHljwDtbwffjtbhjfwNfHmwwf
SZLVdsvrrdFdBcdZvsBdDCNtbmftNwfNbNhCNvtz
MLzzddLsQRppRlQGPq
PDDpdJgtpppGgttgdGdgJFzLjVcvVnnCTrVrRPTLvwnTTC
ZSbHBsSNlZcsfNnnvRrnVjrHwvCC
NmSmsfsfhmzcDmctJW
NbrLfrrLqpqWQHtBzbFttJgcgB
CmwjPPjjjShPvljwvwwjPBFttBtcHzFJcHTRHJRRmT
CCljjDGhvPCVdVSCdPvrrNfnnQsGqMpqqMqnFW
bdPdbcDZlddsZbHjrrgrmZmCZhCGjv
BffLfLVFVMMBRfwMpfzhFGFhGWvWvrhNvvNj
RMBpRSnffBJjSbJqdPHsDcbqtl
BgwGwDDZttDDTNND
WzNNnFRWFtTFlFsh
WJjPpPqqzWRbrqnNqvVvgvvdcBwgdrVBZG
FFbMVMFPvJppgvcvrZMjHlCJWHmHHBlqhCmqChCl
RGQVdVVLnLsQnQnnqWBlBmDRBDWWlhBD
SftLzQndGfVgFfjvvM
npvLlFLTWWqdLnJCmBmmpjQjjmjB
tfgDwzwVVVVtgtrsJtrbjSPQjQmjNBCNBNhPHDHC
ggVzVtMRgzMrvJLqFnnRnnRT
gZFZssWgNZTDwHDWzsFwWDQMMpqqpBPMjFtMPSQFqqqM
vrmvhdnVvQpftStnMN
JCdLddhhdJdcCdrrmCGhlgNsWDWDwWsgwHgHLZHW
vSsSGjSPvjvRSGpFprFbqFpppRfp
ZdmlndtBZbwrwfpWFn
JmdHdBBHtgllZldBhJZldLLBjPVQTfvGPNzQQSjjzPgTGNTs
TjTjBjVrTsLRRrMBsMMgzLqGGqgQHQdCQGgpgd
nbZcmNnPNcbNftvhlhZpgQgCqdSpgCHCqPFzSH
WfcNvtmmNmQlvNcbsWWjMwMVMRDVMDJM
hHHnfZSwHDgHcfclSGSnvrnvBCvWWntvzvzbWWVq
dTJTmspFTsFdRRLvtvLzvvVqtPVtrb
VFMNFpRJNTppTpsJVRcMGgfGfgZwghgGfcGh
lLGvwsMJLCMVnTrCrVdHRd
tbzqtDNNBpNWBtqzfRrFFnrVTTdrQVSVGp
tztfzmfzzPDzgWNNBbhGMlJLsvhJJjGJhGmM
rHrVJQVQVJLggDQQLbTvdCCSTdWLLLbCbS
pNtnwPthmZGRpmPFtqbMSzqffFSdTvbSzW
GpvvshwtmwsZDljjssHjVBVj
SmhJdtJhhzQSrzVhtQbtBRNfnFNSnDNGRfFGGMgR
lHwqPjqwTjLHCWLvPpvNrNMvnNGNfNGNBffGRN
lCrPTrwpPZWlqPlqpWWqZjsmJzbzVtVhhdsJcQdddZVJ
QqpCWHdQdVQlWcQCqcfRjnZZZPDnSPqPhhqZ
tmmzgWGgwJwwStSZZDRnZssR
GbFbLLvgmMMwGgmLCrppQrWVlWrFTHHd
qdqCgSVdVSVqfwsdZhpJspZsph
RjZBbmRlrlmmJwLNNNhLpwhB
vZlRrtTZCzCMfSPT
JBjhCNwjrlJlHJJRsscZrTcvLLgTsLPP
dDztmntCSgbLgqTzgc
fGVWnSMFtVGMNNNQllBjWHJC
dSDhVVdVZtnSgHQGThQvFNQQqF
LcfLRpMpcBpbrJfsbsscBNWRNPRGHvqPdTGPGqPWFH
spmrCcrdJJpLLmcmLLLDlzSzCCjVwZVtgnSZzn
ZJtgPTHtZPZQGbtNzzprVWWVrbrpCD
BRlfcRmBhSMVBqSVfBvNWrDrjWCjjzprCDCl
LfMRSmfqfSLcnnMqVSfccnhZwJFHZFTGZQGFLggwZTZGPJ
BChWddRRRcfmDbfhDP
MgpMFFsvMfGwvLgPjQPzPPmDcztDtw
NFgJpqvpLgqFnWWVNSfnNSCR
zMMMRmMfJpfhpzQJLMVtjtjPntgBtlZlVgJP
SdNbZvZbvbHTNbZbSWTdrTVBglBDlWBjDPDgntPqBDPt
rrNcrFwNdSrfzwMzZMLQQs
JPmCSfHTGJdTCbHgpgqLgRhghhffhg
ZWSSFVSVFQghQvwpphgh
lsDtZjVMMSdCNdbGCbjb
PBQPvDvVVRvQDqLDzJTlzwjz
tGcZTcdgGcncdrFrsTjzJSJqJqqwHSzzSZwq
CgtgdFgcFCMnMgsGFGGQWPQpQTCBvbNpNVVWVT
FHVFWMHMgVhnLWWMpnppfcdZNcPplnfn
RSvSCBSqGgDRjqCpPlPpppTpPjlcNP
zRzsGgJDqJwLVWVFwM
ThhWhNthVWTWqbWbFWbTdBtWSdMlHSlGlCGCdsCMClmnSlMn
DPPpvvfDHfLgDHvzpvPDsssMsmmzsMcClScMMcGG
rgPLHHJJHgZfvvZQQZfrpfFqBBwFBTNTBNBwtthQwVqt
JJgSWDSmSDQCFrhbRLSwLS
VznqzVNsMsZLdqslbRChtbHdHRrwHb
LVfNLMsLTmWDpBpf
cbTsnNpcnnchllFQlMRgJhRP
WddmdMVSBMWSBWjwCJVCPRwPFCQRRC
DWSSfjdSrTpDnHHMbZ
fgsVqqwQQtHhCrDfJH
pvbnBZWBbvWbTdthrJbDmqrmHq
TvZSNSNNSvFMBpqpnLnTBZBFGQwlQFggcFzcVGRlswsRll
zfMcQHzPtRNvlllc
BLnMhbZMLJLNNVtCdNgZgt
BJGFpqMBhBLLMqnwBhbrbhLssjFzssjfDzsmFmjmQFQPjT
JPBJPnpBFrqBJHtjlCjHJcCthM
wQZggQWQGfZFVmmGfDRjlvvNcvlNDhcNttlctt
SfmWfwVFwVGZWQVGSTdTbBpTPSqBbLnnrn
RJqBRJbqpqqJGvqHMmcfczfcjvHQfm
llgVnSWSlsssTnlWjhTcsZMcZcZMZMcccMNmcH
TnFhhllnWCnVCTllLnhhVSrbDDPrdpjRRqjBPRBBpbFJ
mRwRRNDjNTqwDNjNnNRTsQLcQWpQWZJLlLpQWs
PMFGCSzzgbBVzCGShVQZcgWsQLLftttQtlZZ
SlGVldCGGbBPMhPCVSBrNNdjRqjDrNmDnmrRwN
mqGGqGHnqGBCMrnGCbbbLgTTFFNNghHNTj
SdRfcsDPPcDdRzWPWltSlscwTSbShhgpQhgbFLbTQFjwhN
DPWWZzzztsDDtfzlscsPdcWZnVMNVqqGJnBrVqCrMrVvZnBJ
ZgglFCrrrlrWCJswHmwRVmFSwSsP
zhzqBLcjjnpzMzjhTtcqnVGbwssVRmqbHNPbwPsVNH
ftBjzLtptRWdvZlQQZQf
nGpsMncVRMGSnfsBllZdppwrTljZrQ
gcgHmtbCthHWhwBFBWZBlWlWrd
bCDDqHhcqbbtqcqtvJMzsGRsvVfPsfnzJV
TclPvSGZsPZRjhjWDgjp
JtnwHFtJqtwfQfgWgRWhdhjtgdRM
JBwnHwgFFqVJrsGmPvNTPsVvSN
ZJnfZNnDNZJLzNntDtDNNzNWTVBPrrvRRGdBcVRfPPcvfdMr
CFgjFmggQSQQmSggVMMvRdTvBVRjrdrc
mbsqQFqFgwwmgmSbwQWWLDWzpLcLnzZzLbLL
PnwSFSLSTwbbHdtstW
RrDZVVfJNZCmDCfVDVlblZHbddtHScbWbMjt
NmzqhzCCqmzffhCCqrhhLLnPvpnTPPgpGTTBSL
ShhfLSDDFMPQddpMrDgNbjzffqqqzgcjbqZR
sCstmwJwVBtmTltVmTVbRbcbcRvqvrZvBRvZbR
VwCnwnVrrrWShWPHHDdQFL
pbpDpWjZMmFCmmmb
jTjtJLJgJncCFmnJFC
LvhvhTQhBSdRNtLNsSszlGrHSGjZDlGf
JrhvTNJJhhCrtVtcrNLwDBSBwqzDwQVbBLQS
RnCgHmHHGMdPsGMfDlDqlSQbQnQQDbzD
RdPMPsmWHmjfMffPcCWrptcprpFTFrFp";
}