- [ ] clarify the attention distribution method
- [ ] test the rumtime of att and proj
- [ ] get the confliction of the array read of pim and normal

with no read overhead:
Stage	total_cycles	pim_cycles	mem_bw_util	
A	33823	0	91.502824	
B	189033	138835	19.023464	
C	158604	92977	83.611936	
D	198390	143315	68.295970	
E	155817	91418	65.245047	
F	99025	0	100.511739	


with read overhead:
Stage	total_cycles	pim_cycles	mem_bw_util	
A	49338	0	62.728526	
B	187861	138142	19.142145	
C	238461	88921	55.611557	
D	242539	146516	55.864160	
E	179764	87773	56.553523	
F	160278	0	62.099446	


other_banks_same_bg act to g_act: rRRD_L or rRRD_S if only on BG
other_bg_same_rank: act to g_act: rRRD_S
same_bank: act to G_ACT: tRC


other_banks_same_bankgroup: g_act to G_ACT: tRC, g_act to act: tRC
same_rank: G_ACT to act: tFAW


no_conflict_act_to_gact = true
no_conflict_gact_to_act = true


Stage	total_cycles	pim_cycles	mem_bw_util	
A	49338	0	62.728526	
B	188133	138174	19.114470	
C	232056	88625	57.146497	
D	238160	146607	56.891323	
E	173149	89222	58.714099	
F	158218	0	62.907981	



no_conflict_act_to_gact = true
no_conflict_gact_to_act = false

Stage	total_cycles	pim_cycles	mem_bw_util	
A	49338	0	62.728526	
B	187861	138142	19.142145	
C	238461	88921	55.611557	
D	242539	146516	55.864160	
E	179764	87773	56.553523	
F	160278	0	62.099446	


no_conflict_act_to_gact = false
no_conflict_gact_to_act = true

Stage	total_cycles	pim_cycles	mem_bw_util	
A	49338	0	62.728526	
B	187312	137782	19.198249	
C	232290	88805	57.088930	
D	236779	147622	57.223138	
E	173228	89182	58.687322	
F	158376	0	62.845223	
