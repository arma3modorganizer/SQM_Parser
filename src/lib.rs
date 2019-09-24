extern crate pest;
#[macro_use]
extern crate pest_derive;


#[cfg(test)]
mod tests {
    use pest::Parser;
    use pest::iterators::Pairs;

    #[derive(Parser)]
    #[grammar = "sqm.pest"]
    pub struct SQMParser;

    #[test]
    fn test_item(){
        let test_arrays = vec![
            "binarizationWanted=0;",
            "version=53;"
        ];

        let result_arrays = vec![
            vec!["binarizationWanted", "0"],
            vec!["version", "53"],
        ];

        let mut i = 0;
        for test_entry in test_arrays {
            let parsed: Result<Pairs<Rule>, pest::error::Error<Rule>> = SQMParser::parse(Rule::item, test_entry);
            let mut actual_vec: Vec<&str> = vec![];
            match parsed {
                Ok(v) => {
                    for x in v {
                        let i = x.into_inner();
                        for x2 in i {
                            actual_vec.push(x2.as_span().as_str())
                        }
                    }
                }
                Err(e) => {
                    panic!(e)
                }
            }
            assert_eq!(result_arrays[i], actual_vec);
            i += 1;
        }
    }

    #[test]
    fn test_array(){
        let test_arrays = vec![
            "pos[]={A,B,C,D};",
            "aside[]={0.77140951,-2.8638169e-008,0.63634229};",
            "addons[]=\
            {\
                \"keko_common\",\
                \"A3_Modules_F_Curator_Curator\",\
                \"A3_Characters_F\"\
            };"
        ];

        let result_arrays = vec![
            vec!["pos", "A", "B", "C", "D"],
            vec!["aside", "0.77140951", "-2.8638169e-008", "0.63634229",],
            vec!["addons", "\"keko_common\"", "\"A3_Modules_F_Curator_Curator\"", "\"A3_Characters_F\"",],
        ];

        let mut i = 0;
        for test_entry in test_arrays {
            let parsed: Result<Pairs<Rule>, pest::error::Error<Rule>> = SQMParser::parse(Rule::array, test_entry);
            let mut actual_vec: Vec<&str> = vec![];
            match parsed {
                Ok(v) => {
                    for x in v {
                        let i = x.into_inner();
                        for x2 in i {
                            actual_vec.push(x2.as_span().as_str())
                        }
                    }
                }
                Err(e) => {
                    panic!(e)
                }
            }
            assert_eq!(result_arrays[i], actual_vec);
            i += 1;
        }
    }

    #[test]
    fn test_mission(){
        let mission_file = "version=53;\
        class EditorData\
        {\
            moveGridStep=1;\
            angleGridStep=0.2617994;\
            scaleGridStep=1;\
            autoGroupingDist=10;\
            toggles=513;\
            class ItemIDProvider\
            {\
                nextID=1082;\
            };\
            class MarkerIDProvider\
            {\
                nextID=4;\
            };\
            class LayerIndexProvider\
            {\
                nextID=31;\
            };\
            class Camera\
            {\
                pos[]={9613.3701,157.46709,13227.728};\
                dir[]={-0.51592207,-0.58537644,0.62542969};\
                up[]={-0.37250006,0.81076103,0.45156556};\
                aside[]={0.77140951,-2.8638169e-008,0.63634229};\
            };\
        };\
        binarizationWanted=0;\
        addons[]=\
        {\
            \"keko_common\",\
	\"A3_Modules_F_Curator_Curator\",\
	\"A3_Characters_F\",\
	\"ravage\",\
	\"A3_Ui_F\",\
	\"A3_Structures_F_Mil_Fortification\",\
	\"A3_Structures_F_Mil_BagFence\",\
	\"A3_Structures_F_Mil_Shelters\",\
	\"CUP_CAMisc\",\
	\"ace_logistics_wirecutter\",\
	\"ace_concertina_wire\",\
	\"rhs_c_btr\",\
	\"rhs_c_kamaz\",\
	\"rhs_c_heavyweapons\",\
	\"rhs_c_troops\",\
	\"A3_Modules_F\",\
	\"A3_Structures_F_Mil_Helipads\",\
	\"rhs_c_a2port_air\",\
	\"A3_Structures_F_Naval_Piers\",\
	\"A3_Boat_F_Boat_Armed_01\",\
	\"A3_Structures_F_Mil_TentHangar\",\
	\"A3_Supplies_F_Heli_CargoNets\",\
	\"blackorder_supplies\",\
	\"A3_Structures_F_Heli_Ind_Machines\",\
	\"A3_Structures_F_Tank_Military_RepairDepot\",\
	\"A3_Armor_F_Tank_MBT_04\",\
	\"ace_realisticnames\",\
	\"A3_Props_F_Tank_Military_TankAcc\",\
	\"A3_Structures_F_Civ_InfoBoards\",\
	\"A3_Weapons_F_Ammoboxes\",\
	\"A3_Structures_F_EPA_Mil_Scrapyard\",\
	\"A3_Structures_F_Items_Vessels\",\
	\"ace_cargo\",\
	\"keko_logistics\",\
	\"A3_Structures_F_Civ_Market\",\
	\"ace_explosives\",\
	\"A3_Structures_F_Civ_Garbage\",\
	\"ace_chemlights\",\
	\"A3_Structures_F_Civ_Constructions\",\
	\"A3_Structures_F_EPA_Items_Food\",\
	\"A3_Structures_F_EPA_Items_Vessels\",\
	\"ace_dragging\",\
	\"A3_Structures_F_Mil_Cargo\",\
	\"A3_Structures_F_EPA_Civ_Constructions\",\
	\"keko_faction_generic\",\
	\"A3_Structures_F_Walls\",\
	\"A3_Structures_F_Civ_Camping\",\
	\"acex_sitting\",\
	\"A3_Props_F_Orange_Humanitarian_Supplies\",\
	\"rhs_c_cars\",\
	\"rhs_c_trucks\",\
	\"rhs_c_a2port_car\",\
	\"A3_Structures_F_Items_Electronics\",\
	\"A3_Structures_F_Enoch_VR_Helpers\",\
	\"keko_loadout\"\
};\
class AddonsMetaData\
{\
	class List\
	{\
		items=38;\
		class Item0\
		{\
			className=\"keko_common\";\
			name=\"KEKO - Common\";\
			author=\"Schwaggot\";\
			url=\"https://github.com/Schwaggot/kellerkompanie-mods\";\
		};\
		class Item1\
		{\
			className=\"A3_Modules_F_Curator\";\
			name=\"Arma 3 Zeus Update - Scripted Modules\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item2\
		{\
			className=\"A3_Characters_F\";\
			name=\"Arma 3 Alpha - Characters and Clothing\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item3\
		{\
			className=\"ravage\";\
			name=\"ravage\";\
			author=\"Haleks\";\
		};\
		class Item4\
		{\
			className=\"A3_Ui_F\";\
			name=\"Arma 3 - User Interface\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item5\
		{\
			className=\"A3_Structures_F_Mil\";\
			name=\"Arma 3 - Military Buildings and Structures\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item6\
		{\
			className=\"CUP_CAMisc\";\
			name=\"CUP_CAMisc\";\
		};\
		class Item7\
		{\
			className=\"ace_logistics_wirecutter\";\
			name=\"ACE3 - Logistics Wire Cutter\";\
			author=\"ACE-Team\";\
			url=\"http://ace3mod.com/\";\
		};\
		class Item8\
		{\
			className=\"ace_concertina_wire\";\
			name=\"ACE3 - Concertina Wire\";\
			author=\"ACE-Team\";\
			url=\"http://ace3mod.com/\";\
		};\
		class Item9\
		{\
			className=\"rhs_c_btr\";\
			name=\"BTR-70 & 80\";\
			author=\"Red Hammer Studios\";\
			url=\"http://www.rhsmods.org/\";\
		};\
		class Item10\
		{\
			className=\"rhs_c_kamaz\";\
			name=\"KamAZ 5350\";\
			author=\"Red Hammer Studios\";\
			url=\"http://www.rhsmods.org/\";\
		};\
		class Item11\
		{\
			className=\"rhs_c_heavyweapons\";\
			name=\"Heavy weapons pack\";\
			author=\"Red Hammer Studios\";\
			url=\"http://www.rhsmods.org/\";\
		};\
		class Item12\
		{\
			className=\"rhs_c_troops\";\
			name=\"AFRF Infantry & Equipment\";\
			author=\"Red Hammer Studios\";\
			url=\"http://www.rhsmods.org/\";\
		};\
		class Item13\
		{\
			className=\"A3_Modules_F\";\
			name=\"Arma 3 Alpha - Scripted Modules\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item14\
		{\
			className=\"rhs_c_a2port_air\";\
			name=\"A2 ported air\";\
			author=\"Red Hammer Studios\";\
			url=\"http://www.rhsmods.org/\";\
		};\
		class Item15\
		{\
			className=\"A3_Structures_F\";\
			name=\"Arma 3 - Buildings and Structures\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item16\
		{\
			className=\"A3_Boat_F\";\
			name=\"Arma 3 Alpha - Boats and Submersibles\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item17\
		{\
			className=\"A3_Supplies_F_Heli\";\
			name=\"Arma 3 Helicopters - Ammoboxes and Supplies\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item18\
		{\
			className=\"blackorder_supplies\";\
			name=\"BlackOrder - Supplies\";\
			author=\"Black Order-Team\";\
			url=\"http://blackordermod.com\";\
		};\
		class Item19\
		{\
			className=\"A3_Structures_F_Heli\";\
			name=\"Arma 3 Helicopters - Buildings and Structures\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item20\
		{\
			className=\"A3_Structures_F_Tank\";\
			name=\"Arma 3 Tank - Buildings and Structures\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item21\
		{\
			className=\"A3_Armor_F_Tank\";\
			name=\"Arma 3 Tank - Armored Land Vehicles\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item22\
		{\
			className=\"A3_Props_F_Tank\";\
			name=\"Arma 3 Tank - Decorative and Mission Objects\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item23\
		{\
			className=\"A3_Weapons_F\";\
			name=\"Arma 3 Alpha - Weapons and Accessories\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item24\
		{\
			className=\"A3_Structures_F_EPA\";\
			name=\"Arma 3 Survive Episode - Buildings and Structures\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item25\
		{\
			className=\"ace_cargo\";\
			name=\"ACE3 - Cargo\";\
			author=\"ACE-Team\";\
			url=\"http://ace3mod.com/\";\
		};\
		class Item26\
		{\
			className=\"keko_logistics\";\
			name=\"keko_logistics\";\
			author=\"Schwaggot\";\
			url=\"https://github.com/Schwaggot/kellerkompanie-mods\";\
		};\
		class Item27\
		{\
			className=\"ace_explosives\";\
			name=\"ACE3 - Explosives\";\
			author=\"ACE-Team\";\
			url=\"http://ace3mod.com/\";\
		};\
		class Item28\
		{\
			className=\"ace_chemlights\";\
			name=\"ace_chemlights\";\
			author=\"ACE-Team\";\
			url=\"http://ace3mod.com/\";\
		};\
		class Item29\
		{\
			className=\"ace_dragging\";\
			name=\"ACE3 - Dragging\";\
			author=\"ACE-Team\";\
			url=\"http://ace3mod.com/\";\
		};\
		class Item30\
		{\
			className=\"keko_faction_generic\";\
			name=\"keko_faction_generic\";\
			author=\"Schwaggot\";\
			url=\"https://github.com/Schwaggot/kellerkompanie-mods\";\
		};\
		class Item31\
		{\
			className=\"acex_sitting\";\
			name=\"ACEX - Sitting\";\
			author=\"ACE-Team\";\
			url=\"http://ace3mod.com/\";\
		};\
		class Item32\
		{\
			className=\"A3_Props_F_Orange\";\
			name=\"Arma 3 Orange - Decorative and Mission Objects\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item33\
		{\
			className=\"rhs_c_cars\";\
			name=\"Tigr vehicles\";\
			author=\"Red Hammer Studios\";\
			url=\"http://www.rhsmods.org/\";\
		};\
		class Item34\
		{\
			className=\"rhs_c_trucks\";\
			name=\"GAZ-66\";\
			author=\"Red Hammer Studios\";\
			url=\"http://www.rhsmods.org/\";\
		};\
		class Item35\
		{\
			className=\"rhs_c_a2port_car\";\
			name=\"A2 ported cars\";\
			author=\"Red Hammer Studios\";\
			url=\"http://www.rhsmods.org/\";\
		};\
		class Item36\
		{\
			className=\"A3_Structures_F_Enoch\";\
			name=\"Arma 3 Enoch - Buildings and Structures\";\
			author=\"Bohemia Interactive\";\
			url=\"https://www.arma3.com\";\
		};\
		class Item37\
		{\
			className=\"keko_loadout\";\
			name=\"KEKO - Loadout\";\
			author=\"Schwaggot\";\
			url=\"https://github.com/Schwaggot/kellerkompanie-mods\";\
		};\
	};\
};\
randomSeed=3370827;\
class ScenarioData\
{\
	author=\"Grille\";\
	onLoadMission=\"www.kellerkompanie.com\";\
	loadScreen=\"\\x\\keko\\addons\\common\\pictures\\intro.paa\";\
	forceRotorLibSimulation=1;\
	disabledAI=1;\
	respawn=3;\
	respawnDelay=10;\
	enableTeamSwitch=0;\
	class Header\
	{\
		gameType=\"COOP\";\
		minPlayers=1;\
		maxPlayers=21;\
	};\
	corpseLimit=20;\
	corpseRemovalMinTime=300;\
	minPlayerDistance=50;\
};\
class CustomAttributes\
{\
	class Category0\
	{\
		name=\"Multiplayer\";\
		class Attribute0\
		{\
			property=\"RespawnTemplates\";\
			expression=\"true\";\
			class Value\
			{\
				class data\
				{\
					class type\
					{\
						type[]=\
						{\
							\"ARRAY\"\
						};\
					};\
					class value\
					{\
						items=1;\
						class Item0\
						{\
							class data\
							{\
								class type\
								{\
									type[]=\
									{\
										\"STRING\"\
									};\
								};\
								value=\"Counter\";\
							};\
						};\
					};\
				};\
			};\
		};\
		nAttributes=1;\
	};\
	class Category1\
	{\
		name=\"Scenario\";\
		class Attribute0\
		{\
			property=\"cba_settings_hash\";\
			expression=\"false\";\
			class Value\
			{\
				class data\
				{\
					class type\
					{\
						type[]=\
						{\
							\"ARRAY\"\
						};\
					};\
					class value\
					{\
						items=4;\
						class Item0\
						{\
							class data\
							{\
								class type\
								{\
									type[]=\
									{\
										\"STRING\"\
									};\
								};\
								value=\"#CBA_HASH#\";\
							};\
						};\
						class Item1\
						{\
							class data\
							{\
								class type\
								{\
									type[]=\
									{\
										\"ARRAY\"\
									};\
								};\
								class value\
								{\
									items=23;\
									class Item0\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"ace_medical_level\";\
										};\
									};\
									class Item1\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"ace_medical_medicsetting\";\
										};\
									};\
									class Item2\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"ace_medical_increasetraininginlocations\";\
										};\
									};\
									class Item3\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"ace_medical_enableadvancedwounds\";\
										};\
									};\
									class Item4\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"ace_medical_enableunconsciousnessai\";\
										};\
									};\
									class Item5\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"ace_medical_preventinstadeath\";\
										};\
									};\
									class Item6\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"ace_medical_medicsetting_pak\";\
										};\
									};\
									class Item7\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"ace_medical_uselocation_pak\";\
										};\
									};\
									class Item8\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"ace_medical_uselocation_surgicalkit\";\
										};\
									};\
									class Item9\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"ace_advanced_ballistics_enabled\";\
										};\
									};\
									class Item10\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"kat_aceairway_enable\";\
										};\
									};\
									class Item11\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"kat_aceairway_deathtimer\";\
										};\
									};\
									class Item12\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"kat_aceairway_probability_obstruction\";\
										};\
									};\
									class Item13\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"kat_aceairway_probability_occluded\";\
										};\
									};\
									class Item14\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"kat_aceairway_checkbox_puking_sound\";\
										};\
									};\
									class Item15\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"zen_common_autoaddobjects\";\
										};\
									};\
									class Item16\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"zen_common_disablegearanim\";\
										};\
									};\
									class Item17\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"kat_acecirculation_enable\";\
										};\
									};\
									class Item18\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"keko_loadout_loadoutfaction\";\
										};\
									};\
									class Item19\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"keko_loot_preventcorpselooting\";\
										};\
									};\
									class Item20\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"keko_unknown_weapon_enabled\";\
										};\
									};\
									class Item21\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"keko_unknown_weapon_briefing\";\
										};\
									};\
									class Item22\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"kat_acebreathing_enable\";\
										};\
									};\
								};\
							};\
						};\
						class Item2\
						{\
							class data\
							{\
								class type\
								{\
									type[]=\
									{\
										\"ARRAY\"\
									};\
								};\
								class value\
								{\
									items=23;\
									class Item0\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=2;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item1\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=2;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item2\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=1;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item3\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=1;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item4\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=0;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item5\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=1;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item6\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item7\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=2;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item8\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=0;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item9\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=1;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item10\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=0;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item11\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1800;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item12\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=0;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item13\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=0;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item14\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=0;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item15\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=1;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item16\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=1;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item17\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=0;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item18\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"STRING\"\
															};\
														};\
														value=\"FactionRussiaEMRSummer\";\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item19\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=0;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item20\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=0;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item21\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=0;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
									class Item22\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"BOOL\"\
															};\
														};\
														value=0;\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"SCALAR\"\
															};\
														};\
														value=1;\
													};\
												};\
											};\
										};\
									};\
								};\
							};\
						};\
						class Item3\
						{\
							class data\
							{\
								nil=1;\
								class type\
								{\
									type[]=\
									{\
										\"ANY\"\
									};\
								};\
							};\
						};\
					};\
				};\
			};\
		};\
		class Attribute1\
		{\
			property=\"Enh_ambientFlyby_speed\";\
			expression=\"missionNamespace setVariable ['Enh_AmbientFlyby_Speed',_value]\";\
			class Value\
			{\
				class data\
				{\
					class type\
					{\
						type[]=\
						{\
							\"STRING\"\
						};\
					};\
					value=\"normal\";\
				};\
			};\
		};\
		class Attribute2\
		{\
			property=\"Enh_Airdrop_Condition\";\
			expression=\"if (isServer) then {missionNamespace setVariable ['Enh_Airdrop_Condition',_value]}\";\
			class Value\
			{\
				class data\
				{\
					class type\
					{\
						type[]=\
						{\
							\"STRING\"\
						};\
					};\
					value=\"false\";\
				};\
			};\
		};\
		class Attribute3\
		{\
			property=\"Enh_Airdrop_Side\";\
			expression=\"if (isServer) then {missionNamespace setVariable ['Enh_Airdrop_Side',_value]}\";\
			class Value\
			{\
				class data\
				{\
					class type\
					{\
						type[]=\
						{\
							\"STRING\"\
						};\
					};\
					value=\"WEST\";\
				};\
			};\
		};\
		class Attribute4\
		{\
			property=\"Enh_dynamicGroups\";\
			expression=\"		if (!is3DEN && isServer && _value && !(allCurators isEqualTo [])) then		{			[] spawn			{				while {true} do				{					{						 _x addCuratorEditableObjects						 [						 	entities [[],['Logic'],true],						  	true						 ];					} count allCurators;					sleep 60;				};			};		}\";\
			class Value\
			{\
				class data\
				{\
					class type\
					{\
						type[]=\
						{\
							\"BOOL\"\
						};\
					};\
					value=1;\
				};\
			};\
		};\
		nAttributes=5;\
	};\
};\
class Mission\
{\
	class Intel\
	{\
		briefingName=\"ChangeMe\";\
		resistanceWest=0;\
		timeOfChanges=1800.0002;\
		startWeather=0.30000001;\
		startWind=0.1;\
		startWaves=0.1;\
		forecastWeather=0.30000001;\
		forecastWind=0.1;\
		forecastWaves=0.1;\
		forecastLightnings=0.1;\
		wavesForced=1;\
		windForced=1;\
		year=2027;\
		month=3;\
		day=20;\
		hour=9;\
		minute=-15;\
		startFogDecay=0.014;\
		forecastFogDecay=0.014;\
		class CustomAttributes\
		{\
			name=\"Intel\";\
			class Attribute0\
			{\
				property=\"Enh_disableGrass\";\
				expression=\"if (((parseNumber _value) > 0) && !is3DEN && hasInterface) then {setTerrainGrid (parseNumber _value)}\";\
				class Value\
				{\
					class data\
					{\
						class type\
						{\
							type[]=\
							{\
								\"STRING\"\
							};\
						};\
						value=\"-1\";\
					};\
				};\
			};\
			nAttributes=1;\
		};\
	};\
	class Entities\
	{\
		items=23;\
		class Item0\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16153.729,443.93765,16447.193};\
				angles[]={6.2801557,0,6.2724767};\
			};\
			name=\"keko_common_3den_initModule\";\
			id=0;\
			type=\"keko_common_moduleInitMission3den\";\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"keko_common_ModuleInit_MissionPictureSubtitle\";\
					expression=\"_this setVariable ['MissionPictureSubtitle',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"www.kellerkompanie.com\";\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"keko_common_ModuleInit_MissionPicture\";\
					expression=\"_this setVariable ['MissionPicture',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"\\x\\keko\\addons\\common\\pictures\\intro.paa\";\
						};\
					};\
				};\
				class Attribute2\
				{\
					property=\"keko_common_ModuleInit_GarbageCollectionCorpses\";\
					expression=\"_this setVariable ['GarbageCollectionCorpses',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute3\
				{\
					property=\"keko_common_ModuleInit_SideRelations\";\
					expression=\"_this setVariable ['SideRelations',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute4\
				{\
					property=\"keko_common_ModuleInit_GarbageCollectionWrecks\";\
					expression=\"_this setVariable ['GarbageCollectionWrecks',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute5\
				{\
					property=\"keko_common_ModuleInit_MissionAuthor\";\
					expression=\"_this setVariable ['MissionAuthor',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"Grille\";\
						};\
					};\
				};\
				class Attribute6\
				{\
					property=\"keko_common_ModuleInit_MissionTitle\";\
					expression=\"_this setVariable ['MissionTitle',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"ChangeMe\";\
						};\
					};\
				};\
				nAttributes=7;\
			};\
		};\
		class Item1\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16060.558,437.19843,16448.814};\
				angles[]={0.20336431,0,0.098533578};\
			};\
			name=\"keko_common_3den_gameMaster\";\
			id=1;\
			type=\"ModuleCurator_F\";\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"ModuleCurator_F_Owner\";\
					expression=\"_this setVariable ['Owner',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"#adminLogged\";\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"ModuleCurator_F_Forced\";\
					expression=\"_this setVariable ['Forced',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute2\
				{\
					property=\"ModuleCurator_F_Name\";\
					expression=\"_this setVariable ['Name',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"Admin\";\
						};\
					};\
				};\
				class Attribute3\
				{\
					property=\"ModuleCurator_F_Addons\";\
					expression=\"_this setVariable ['Addons',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=3;\
						};\
					};\
				};\
				nAttributes=4;\
			};\
		};\
		class Item2\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16057.316,350.56607,16646.498};\
				angles[]={0.26691997,0,0.32024825};\
			};\
			name=\"HC1\";\
			isPlayable=1;\
			description=\"HC1\";\
			id=2;\
			type=\"HeadlessClient_F\";\
		};\
		class Item3\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16157.779,374.67917,16645.688};\
				angles[]={0.2752206,0,0.63143545};\
			};\
			name=\"HC2\";\
			isPlayable=1;\
			description=\"HC2\";\
			id=3;\
			type=\"HeadlessClient_F\";\
		};\
		class Item4\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16250.949,408.72516,16646.498};\
				angles[]={6.1427512,0,0.13335171};\
			};\
			name=\"HC3\";\
			isPlayable=1;\
			description=\"HC3\";\
			id=4;\
			type=\"HeadlessClient_F\";\
		};\
		class Item5\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16055.696,435.37689,16341.061};\
			};\
			id=6;\
			type=\"Ravage_ai\";\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"Ravage_ai_ai_huntersHost2_m\";\
					expression=\"_this setVariable ['ai_huntersHost2_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"Ravage_ai_SpawnAI_traders_m\";\
					expression=\"_this setVariable ['SpawnAI_traders_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute2\
				{\
					property=\"Ravage_ai_ai_condition_m\";\
					expression=\"_this setVariable ['ai_condition_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"True\";\
						};\
					};\
				};\
				class Attribute3\
				{\
					property=\"Ravage_ai_ai_remoteZeus_m\";\
					expression=\"_this setVariable ['ai_remoteZeus_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute4\
				{\
					property=\"Ravage_ai_ai_camps_m\";\
					expression=\"_this setVariable ['ai_camps_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute5\
				{\
					property=\"Ravage_ai_aiPopulationFactor_m\";\
					expression=\"_this setVariable ['aiPopulationFactor_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute6\
				{\
					property=\"Ravage_ai_SpawnAI_host1_m\";\
					expression=\"_this setVariable ['SpawnAI_host1_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"WEST\";\
						};\
					};\
				};\
				class Attribute7\
				{\
					property=\"Ravage_ai_SpawnAI_friendly_m\";\
					expression=\"_this setVariable ['SpawnAI_friendly_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"none\";\
						};\
					};\
				};\
				class Attribute8\
				{\
					property=\"Ravage_ai_ai_distance_m\";\
					expression=\"_this setVariable ['ai_distance_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[400, 1000]\";\
						};\
					};\
				};\
				class Attribute9\
				{\
					property=\"Ravage_ai_SpawnAI_host2_m\";\
					expression=\"_this setVariable ['SpawnAI_host2_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"RESISTANCE\";\
						};\
					};\
				};\
				class Attribute10\
				{\
					property=\"Ravage_ai_custom_aiveh_list_m\";\
					expression=\"_this setVariable ['custom_aiveh_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute11\
				{\
					property=\"Ravage_ai_ambientChat_m\";\
					expression=\"_this setVariable ['ambientChat_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute12\
				{\
					property=\"Ravage_ai_SpawnAI_renegades_m\";\
					expression=\"_this setVariable ['SpawnAI_renegades_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute13\
				{\
					property=\"Ravage_ai_ai_huntersHost1_m\";\
					expression=\"_this setVariable ['ai_huntersHost1_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=15;\
						};\
					};\
				};\
				class Attribute14\
				{\
					property=\"Ravage_ai_carPatrols_m\";\
					expression=\"_this setVariable ['carPatrols_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				nAttributes=15;\
			};\
		};\
		class Item6\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16262.202,432.57251,16340.168};\
			};\
			id=8;\
			type=\"Ravage_atmosphere\";\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"Ravage_atmosphere_breathfog_m\";\
					expression=\"_this setVariable ['breathfog_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"Ravage_atmosphere_arti_lights_m\";\
					expression=\"_this setVariable ['arti_lights_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=-1;\
						};\
					};\
				};\
				class Attribute2\
				{\
					property=\"Ravage_atmosphere_weather_m\";\
					expression=\"_this setVariable ['weather_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute3\
				{\
					property=\"Ravage_atmosphere_custom_st_m\";\
					expression=\"_this setVariable ['custom_st_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute4\
				{\
					property=\"Ravage_atmosphere_atmo_m\";\
					expression=\"_this setVariable ['atmo_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute5\
				{\
					property=\"Ravage_atmosphere_sandGust_m\";\
					expression=\"_this setVariable ['sandGust_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute6\
				{\
					property=\"Ravage_atmosphere_music_m\";\
					expression=\"_this setVariable ['music_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				nAttributes=7;\
			};\
		};\
		class Item7\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16451.904,436.97659,16342.323};\
			};\
			id=9;\
			type=\"Ravage_debug\";\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"Ravage_debug_debugAIMarkers_m\";\
					expression=\"_this setVariable ['debugAIMarkers_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"Ravage_debug_debugRadMarkers_m\";\
					expression=\"_this setVariable ['debugRadMarkers_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute2\
				{\
					property=\"Ravage_debug_debugHordeMarkers_m\";\
					expression=\"_this setVariable ['debugHordeMarkers_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute3\
				{\
					property=\"Ravage_debug_debugBlacklistMarkers_m\";\
					expression=\"_this setVariable ['debugBlacklistMarkers_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				nAttributes=4;\
			};\
		};\
		class Item8\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16352.742,436.50266,16340.886};\
			};\
			id=10;\
			type=\"Ravage_gearPool\";\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"Ravage_gearPool_rhs_m\";\
					expression=\"_this setVariable ['rhs_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"Ravage_gearPool_stock_m\";\
					expression=\"_this setVariable ['stock_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute2\
				{\
					property=\"Ravage_gearPool_gm_m\";\
					expression=\"_this setVariable ['gm_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute3\
				{\
					property=\"Ravage_gearPool_custom_unif1_list_m\";\
					expression=\"_this setVariable ['custom_unif1_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute4\
				{\
					property=\"Ravage_gearPool_frith_m\";\
					expression=\"_this setVariable ['frith_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute5\
				{\
					property=\"Ravage_gearPool_g_presets_m\";\
					expression=\"_this setVariable ['g_presets_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=2;\
						};\
					};\
				};\
				class Attribute6\
				{\
					property=\"Ravage_gearPool_custom_itemCommon_list_m\";\
					expression=\"_this setVariable ['custom_itemCommon_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute7\
				{\
					property=\"Ravage_gearPool_cup_m\";\
					expression=\"_this setVariable ['cup_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=3;\
						};\
					};\
				};\
				class Attribute8\
				{\
					property=\"Ravage_gearPool_custom_weap3_list_m\";\
					expression=\"_this setVariable ['custom_weap3_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute9\
				{\
					property=\"Ravage_gearPool_zed_g_preset_m\";\
					expression=\"_this setVariable ['zed_g_preset_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute10\
				{\
					property=\"Ravage_gearPool_debug\";\
					expression=\"_this setVariable ['debug',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute11\
				{\
					property=\"Ravage_gearPool_custom_itemRare_list_m\";\
					expression=\"_this setVariable ['custom_itemRare_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[\"\"MineDetector\"\",\"\"APERSBoundingMine_Range_Mag\"\",\"\"APERSMine_Range_Mag\"\",\"\"APERSMineDispenser_Mag\"\",\"\"APERSTripMine_Wire_Mag\"\"]\";\
						};\
					};\
				};\
				class Attribute12\
				{\
					property=\"Ravage_gearPool_thai_m\";\
					expression=\"_this setVariable ['thai_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute13\
				{\
					property=\"Ravage_gearPool_hi_uniforms_m\";\
					expression=\"_this setVariable ['hi_uniforms_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=15;\
						};\
					};\
				};\
				class Attribute14\
				{\
					property=\"Ravage_gearPool_custom_vest_list_m\";\
					expression=\"_this setVariable ['custom_vest_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute15\
				{\
					property=\"Ravage_gearPool_custom_weap4_list_m\";\
					expression=\"_this setVariable ['custom_weap4_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute16\
				{\
					property=\"Ravage_gearPool_ifa3_m\";\
					expression=\"_this setVariable ['ifa3_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute17\
				{\
					property=\"Ravage_gearPool_custom_head_list_m\";\
					expression=\"_this setVariable ['custom_head_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute18\
				{\
					property=\"Ravage_gearPool_custom_weap1_list_m\";\
					expression=\"_this setVariable ['custom_weap1_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute19\
				{\
					property=\"Ravage_gearPool_custom_back_list_m\";\
					expression=\"_this setVariable ['custom_back_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute20\
				{\
					property=\"Ravage_gearPool_custom_facewear_list_m\";\
					expression=\"_this setVariable ['custom_facewear_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute21\
				{\
					property=\"Ravage_gearPool_custom_unif0_list_m\";\
					expression=\"_this setVariable ['custom_unif0_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				class Attribute22\
				{\
					property=\"Ravage_gearPool_custom_weap2_list_m\";\
					expression=\"_this setVariable ['custom_weap2_list_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[]\";\
						};\
					};\
				};\
				nAttributes=23;\
			};\
		};\
		class Item9\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16056.693,426.86496,16252.503};\
			};\
			id=12;\
			type=\"Ravage_saveSystem\";\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"Ravage_saveSystem_spsave_m\";\
					expression=\"_this setVariable ['spsave_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"Ravage_saveSystem_mpsave_m\";\
					expression=\"_this setVariable ['mpsave_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				nAttributes=2;\
			};\
		};\
		class Item10\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16150.825,422.70963,16248.19};\
			};\
			id=13;\
			type=\"Ravage_settings\";\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"Ravage_settings_vehCache_m\";\
					expression=\"_this setVariable ['vehCache_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"Ravage_settings_dynamicAnim_m\";\
					expression=\"_this setVariable ['dynamicAnim_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute2\
				{\
					property=\"Ravage_settings_clean_m\";\
					expression=\"_this setVariable ['clean_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute3\
				{\
					property=\"Ravage_settings_advanimal_m\";\
					expression=\"_this setVariable ['advanimal_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute4\
				{\
					property=\"Ravage_settings_looters_m\";\
					expression=\"_this setVariable ['looters_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute5\
				{\
					property=\"Ravage_settings_advdrivers_m\";\
					expression=\"_this setVariable ['advdrivers_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				class Attribute6\
				{\
					property=\"Ravage_settings_holster_m\";\
					expression=\"_this setVariable ['holster_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute7\
				{\
					property=\"Ravage_settings_timeFactor_m\";\
					expression=\"_this setVariable ['timeFactor_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=1;\
						};\
					};\
				};\
				nAttributes=8;\
			};\
		};\
		class Item11\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={16254.299,422.58313,16250.347};\
			};\
			id=14;\
			type=\"Ravage_survival\";\
			atlOffset=-3.0517578e-005;\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"Ravage_survival_RadWater_m\";\
					expression=\"_this setVariable ['RadWater_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"Ravage_survival_Rad_m\";\
					expression=\"_this setVariable ['Rad_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute2\
				{\
					property=\"Ravage_survival_thirstFactor_m\";\
					expression=\"_this setVariable ['thirstFactor_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute3\
				{\
					property=\"Ravage_survival_hungerFactor_m\";\
					expression=\"_this setVariable ['hungerFactor_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute4\
				{\
					property=\"Ravage_survival_RadRain_m\";\
					expression=\"_this setVariable ['RadRain_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				class Attribute5\
				{\
					property=\"Ravage_survival_rvgbeds_m\";\
					expression=\"_this setVariable ['rvgbeds_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"STRING\"\
								};\
							};\
							value=\"[\"\"rvg_dome_tent.p3d\"\",\"\"rvg_dome_tent_blu.p3d\"\",\"\"rvg_dome_tent_grn.p3d\"\",\"\"sofa_01_f.p3d\"\",\"\"postel_panelak2.p3d\"\",\"\"woodenbed_01_f.p3d\"\",\"\"bed_big_a.p3d\"\",\"\"bed_husbands.p3d\"\",\"\"vojenska_palanda.p3d\"\",\"\"postel_manz_kov.p3d\"\",\"\"postel_panelak1.p3d\"\",\"\"sleeping_bag_blue_f.p3d\"\",\"\"sleeping_bag_brown_f.p3d\"\",\"\"tenta_f.p3d\"\",\"\"tentdome_f.p3d\"\",\"\"gm_euro_furniture_bed_01.p3d\"\",\"\"gm_euro_furniture_bed_02.p3d\"\"]\";\
						};\
					};\
				};\
				nAttributes=6;\
			};\
		};\
		class Item12\
		{\
			dataType=\"Marker\";\
			position[]={4592.3306,339.06,10247.115};\
			name=\"marker_1\";\
			text=\"Ex. NATO - Base\";\
			type=\"hd_warning\";\
			colorName=\"ColorRed\";\
			angle=11.842024;\
			id=51;\
		};\
		class Item13\
		{\
			dataType=\"Marker\";\
			position[]={4582.0098,339.06,10236.443};\
			name=\"marker_2_nato_base\";\
			markerType=\"ELLIPSE\";\
			type=\"ellipse\";\
			colorName=\"ColorRed\";\
			fillName=\"FDiagonal\";\
			a=1500;\
			b=1000;\
			angle=52.76199;\
			id=52;\
		};\
		class Item14\
		{\
			dataType=\"Layer\";\
			name=\"Batarov\";\
			class Entities\
			{\
				items=2;\
				class Item0\
				{\
					dataType=\"Layer\";\
					name=\"Empty\";\
					class Entities\
					{\
						items=29;\
						class Item0\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9518.5186,121.54796,13255.005};\
								angles[]={6.2806721,1.5662272,0.17567205};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=256;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item1\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9537.207,124.45965,13245.822};\
								angles[]={6.278194,1.5978867,0.1732446};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=247;\
							type=\"Land_BagFence_Long_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item2\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9537.2539,124.48191,13248.634};\
								angles[]={6.278182,1.5978867,0.17324667};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=248;\
							type=\"Land_BagFence_Long_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item3\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9536.6514,124.38914,13251.167};\
								angles[]={6.278182,3.9662857,0.17324667};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=249;\
							type=\"Land_BagFence_Round_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item4\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9532.7578,124.42149,13245.826};\
								angles[]={6.278194,1.5926423,0.17567001};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=259;\
							type=\"CamoNet_BLUFOR_open_F\";\
							atlOffset=7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item5\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9539.6729,125.08756,13251.113};\
								angles[]={0,1.5322479,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=262;\
							type=\"Danger\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item6\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9538.2266,124.98263,13247.583};\
								angles[]={6.278182,1.5896357,0.17324667};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=260;\
							type=\"Land_Razorwire_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item7\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9526.1182,128.31,13270.381};\
								angles[]={0.0049916417,2.3072729,0.10955431};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=270;\
							type=\"rhs_btr80a_vv\";\
							atlOffset=-0.0085601807;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"rhs_decalVV_type\";\
									expression=\"_this setVariable ['rhs_decalVV_type', _value];\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Platoon\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"rhs_decalArmy\";\
									expression=\"if(parseNumber _value >= 0)then{ [_this, [ [ 'Label', cBTRBackArmSymPlaces,  _this getVariable ['rhs_decalArmy_type','Army'],call compile _value] ] ] call rhs_fnc_decalsInit};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"-1\";\
										};\
									};\
								};\
								class Attribute2\
								{\
									property=\"crate_r2_unhide\";\
									expression=\"[_this,_value,'crate_r2_unhide'] call rhs_fnc_setHabarEden\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"SCALAR\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute3\
								{\
									property=\"rhs_decalFront_type\";\
									expression=\"_this setVariable ['rhs_decalFront_type', _value];\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Army\";\
										};\
									};\
								};\
								class Attribute4\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute5\
								{\
									property=\"rhs_decalNumber_type\";\
									expression=\"if(_value != 'NoChange')then{ _this setVariable ['rhs_decalNumber_type', _value];[_this,[['Number', cBTR3NumberPlaces, _value]]] call rhs_fnc_decalsInit}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"NoChange\";\
										};\
									};\
								};\
								class Attribute6\
								{\
									property=\"crate_l3_unhide\";\
									expression=\"[_this,_value,'crate_l3_unhide'] call rhs_fnc_setHabarEden\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"SCALAR\"\
												};\
											};\
											value=1;\
										};\
									};\
								};\
								class Attribute7\
								{\
									property=\"rhs_decalPlatoon\";\
									expression=\"if(parseNumber _value >= 0)then{ [_this, [ [ 'Label', cBTRPlnSymPlaces,  _this getVariable ['rhs_decalPlatoon_type','Platoon'],call compile _value] ] ] call rhs_fnc_decalsInit};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"-1\";\
										};\
									};\
								};\
								class Attribute8\
								{\
									property=\"rhs_decalArmy_type\";\
									expression=\"_this setVariable ['rhs_decalArmy_type', _value];\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Army\";\
										};\
									};\
								};\
								class Attribute9\
								{\
									property=\"rhs_decalFront\";\
									expression=\"if(parseNumber _value >= 0)then{ [_this, [ [ 'Label', cBTROMONSymbolPlaces,  _this getVariable ['rhs_decalFront_type','Army'],call compile _value] ] ] call rhs_fnc_decalsInit};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"-1\";\
										};\
									};\
								};\
								class Attribute10\
								{\
									property=\"rhs_externalMount\";\
									expression=\"[_this,_value] call rhs_fnc_lockTop\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute11\
								{\
									property=\"crate_l1_unhide\";\
									expression=\"[_this,_value,'crate_l1_unhide'] call rhs_fnc_setHabarEden\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"SCALAR\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute12\
								{\
									property=\"rhs_decalPlatoon_type\";\
									expression=\"_this setVariable ['rhs_decalPlatoon_type', _value];\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Platoon\";\
										};\
									};\
								};\
								class Attribute13\
								{\
									property=\"rhs_decalRightTurret\";\
									expression=\"if(parseNumber _value >= 0)then{ [_this, [ [ 'Label', cBTRRightGvardSymPlaces,  _this getVariable ['rhs_decalRightTurret_type','Honor'],call compile _value] ] ] call rhs_fnc_decalsInit};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"-1\";\
										};\
									};\
								};\
								class Attribute14\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute15\
								{\
									property=\"rhs_decalRightTurret_type\";\
									expression=\"_this setVariable ['rhs_decalRightTurret_type', _value];\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Honor\";\
										};\
									};\
								};\
								class Attribute16\
								{\
									property=\"wheel_1_unhide\";\
									expression=\"[_this,_value,'wheel_1_unhide'] call rhs_fnc_setHabarEden\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"SCALAR\"\
												};\
											};\
											value=1;\
										};\
									};\
								};\
								class Attribute17\
								{\
									property=\"crate_r1_unhide\";\
									expression=\"[_this,_value,'crate_r1_unhide'] call rhs_fnc_setHabarEden\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"SCALAR\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute18\
								{\
									property=\"rhs_decalVV\";\
									expression=\"if(parseNumber _value >= 0)then{ [_this, [ [ 'Label', cBTRVVLetterSymPlaces,  _this getVariable ['rhs_decalVV_type','Platoon'],call compile _value] ] ] call rhs_fnc_decalsInit};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"-1\";\
										};\
									};\
								};\
								class Attribute19\
								{\
									property=\"crate_l2_unhide\";\
									expression=\"[_this,_value,'crate_l2_unhide'] call rhs_fnc_setHabarEden\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"SCALAR\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute20\
								{\
									property=\"ammoBox\";\
									expression=\"[_this,_value] call bis_fnc_initAmmoBox;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"[[[[\"\"rhs_weap_ak74m\"\",\"\"rhs_weap_rpg26\"\",\"\"rhs_weap_rpg7\"\",\"\"FirstAidKit\"\",\"\"Medikit\"\"],[2,2,1,4,1]],[[\"\"rhs_30Rnd_545x39_7N10_AK\"\",\"\"rhs_10Rnd_762x54mmR_7N1\"\",\"\"rhs_100Rnd_762x54mmR\"\",\"\"rhs_mag_rdg2_white\"\",\"\"rhs_mag_rgd5\"\",\"\"rhs_VOG25\"\",\"\"rhs_VG40OP_white\"\",\"\"rhs_GRD40_White\"\",\"\"rhs_rpg26_mag\"\",\"\"rhs_rpg7_OG7V_mag\"\"],[30,10,3,2,9,20,5,5,2,2]],[[],[]],[[\"\"rhs_sidor\"\",\"\"rhs_rpg\"\"],[7,1]]],false]\";\
										};\
									};\
								};\
								nAttributes=21;\
							};\
						};\
						class Item8\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9523.9512,128.44037,13272.823};\
								angles[]={0.052447144,2.2610564,0.12927654};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=272;\
							type=\"CamoNet_BLUFOR_big_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item9\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9249.0801,81.894707,13249.883};\
								angles[]={0.074858427,0,0};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=285;\
							type=\"rhs_kamaz5350_flatbed_vv\";\
							atlOffset=-0.0071182251;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"rhs_decalNumber_type\";\
									expression=\"_this setVariable ['rhs_decalNumber_type', _value];[_this,[['Number', [5,6,7,8], _value]]] call rhs_fnc_decalsInit\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Default\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"rhs_decalPlatoon_type\";\
									expression=\"_this setVariable ['rhs_decalPlatoon_type', _value];\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Army\";\
										};\
									};\
								};\
								class Attribute2\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute3\
								{\
									property=\"ammoBox\";\
									expression=\"[_this,_value] call bis_fnc_initAmmoBox;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"[[[[\"\"FirstAidKit\"\"],[10]],[[],[]],[[],[]],[[],[]]],false]\";\
										};\
									};\
								};\
								class Attribute4\
								{\
									property=\"rhs_decalArmy_type\";\
									expression=\"_this setVariable ['rhs_decalArmy_type', _value];\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Army\";\
										};\
									};\
								};\
								class Attribute5\
								{\
									property=\"rhs_decalArmy\";\
									expression=\"if(parseNumber _value >= 0)then{ [_this, [ [ 'Label', [9],  _this getVariable ['rhs_decalArmy_type','Army'],call compile _value] ] ] call rhs_fnc_decalsInit};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"-1\";\
										};\
									};\
								};\
								class Attribute6\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute7\
								{\
									property=\"rhs_decalPlatoon\";\
									expression=\"if(parseNumber _value >= 0)then{ [_this, [ [ 'Label', [10],  _this getVariable ['rhs_decalPlatoon_type','Army'],call compile _value] ] ] call rhs_fnc_decalsInit};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"-1\";\
										};\
									};\
								};\
								nAttributes=8;\
							};\
						};\
						class Item10\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9252.6758,80.289787,13255.798};\
								angles[]={0,1.6192034,0.0050033992};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=288;\
							type=\"Land_HBarrierWall_corner_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item11\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9253.7559,80.295189,13245.426};\
								angles[]={0,6.2760224,0.0050033992};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=289;\
							type=\"Land_HBarrierWall_corner_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item12\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9254.5723,80.308365,13240.942};\
								angles[]={0.0024897563,1.550539,0.0050033992};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=293;\
							type=\"Land_HBarrierWall_corner_F\";\
							atlOffset=0.001449585;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item13\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9252.2334,80.446075,13260.343};\
								angles[]={6.2357211,0.02304882,6.2631893};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=294;\
							type=\"Land_HBarrierWall_corner_F\";\
							atlOffset=0.058052063;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item14\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9248.8525,80.138367,13240.314};\
								angles[]={6.2257471,3.1341221,0.054945603};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=290;\
							type=\"Land_HBarrierWall6_F\";\
							atlOffset=-7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item15\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9247.4473,80.257927,13258.804};\
								angles[]={6.1884699,5.7223988,0.057436496};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=292;\
							type=\"Land_HBarrierWall6_F\";\
							atlOffset=-4.5776367e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item16\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9257.6885,80.322273,13243.532};\
								angles[]={6.278194,1.5597805,0.01749678};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=295;\
							type=\"Land_Razorwire_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item17\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9255.7783,80.29615,13257.352};\
								angles[]={0.0025135824,1.554273,6.2631893};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=296;\
							type=\"Land_Razorwire_F\";\
							atlOffset=3.0517578e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item18\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9257.0654,80.323051,13235.622};\
								angles[]={0,1.6991459,0.0075011365};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=297;\
							type=\"Land_Razorwire_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item19\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9252.6211,80.562157,13264.319};\
								angles[]={6.270689,0.52015203,0.0049914722};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=298;\
							type=\"Land_Razorwire_F\";\
							atlOffset=-7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item20\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9245.2783,80.117767,13266.949};\
								angles[]={0.022495884,0.0067719049,0.07734675};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=299;\
							type=\"Land_Razorwire_F\";\
							atlOffset=-7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item21\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9222.4922,78.461037,13257.18};\
								angles[]={0.010000871,0.0067719049,0.06739822};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=300;\
							type=\"Land_Razorwire_F\";\
							atlOffset=-0.0104599;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item22\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9230.6533,79.069038,13257.444};\
								angles[]={0.015002378,0.0067719049,0.077345207};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=301;\
							type=\"Land_Razorwire_F\";\
							atlOffset=7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item23\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9238.9824,79.77507,13257.674};\
								angles[]={0.019996032,0.0067719049,0.084795304};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=302;\
							type=\"Land_Razorwire_F\";\
							atlOffset=-2.2888184e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item24\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9214.0322,77.934471,13257.487};\
								angles[]={0.0049914722,0.0067719049,0.057436496};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=305;\
							type=\"Land_Razorwire_F\";\
							atlOffset=-7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item25\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9223.6963,78.771072,13238.721};\
								angles[]={0.022495884,0.0067719049,0.074860819};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=306;\
							type=\"Land_Razorwire_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item26\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9231.8574,79.526566,13238.985};\
								angles[]={0.024993783,0.0067719049,0.097192109};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=307;\
							type=\"Land_Razorwire_F\";\
							atlOffset=3.0517578e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item27\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9240.1865,80.095535,13239.215};\
								angles[]={0.084797412,0.0067719049,0.019996032};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=308;\
							type=\"Land_Razorwire_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item28\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9215.2363,78.135147,13239.028};\
								angles[]={0.010000871,0.0067719049,0.064907812};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=309;\
							type=\"Land_Razorwire_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
					};\
					id=257;\
					atlOffset=0.05014801;\
				};\
				class Item1\
				{\
					dataType=\"Layer\";\
					name=\"REDFOR\";\
					class Entities\
					{\
						items=2;\
						class Item0\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9535.085,125.31757,13247.128};\
								angles[]={6.278194,1.5366216,0.17567001};\
							};\
							side=\"East\";\
							flags=6;\
							class Attributes\
							{\
								init=\"call{this disableAI \"\"PATH\"\";}\";\
							};\
							id=273;\
							type=\"rhs_KORD_high_MSV\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item1\
						{\
							dataType=\"Group\";\
							side=\"East\";\
							class Entities\
							{\
								items=2;\
								class Item0\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9536.0322,123.88548,13255.671};\
										angles[]={0,1.6863453,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										init=\"call{this disableAI \"\"PATH\"\";}\";\
									};\
									id=276;\
									type=\"rhs_mvd_izlom_sergeant\";\
									atlOffset=0.00029754639;\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"speaker\";\
											expression=\"_this setspeaker _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"RHS_Male03RUS\";\
												};\
											};\
										};\
										class Attribute2\
										{\
											property=\"pitch\";\
											expression=\"_this setpitch _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"SCALAR\"\
														};\
													};\
													value=0.97000003;\
												};\
											};\
										};\
										class Attribute3\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=4;\
									};\
								};\
								class Item1\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9535.3682,123.72497,13247.186};\
									};\
									side=\"East\";\
									flags=7;\
									class Attributes\
									{\
									};\
									id=275;\
									type=\"rhs_mvd_izlom_sergeant\";\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"speaker\";\
											expression=\"_this setspeaker _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"RHS_Male01RUS\";\
												};\
											};\
										};\
										class Attribute2\
										{\
											property=\"pitch\";\
											expression=\"_this setpitch _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"SCALAR\"\
														};\
													};\
													value=0.97000003;\
												};\
											};\
										};\
										class Attribute3\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										class Attribute4\
										{\
											property=\"face\";\
											expression=\"_this setface _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"\";\
												};\
											};\
										};\
										nAttributes=5;\
									};\
								};\
							};\
							class Attributes\
							{\
							};\
							class CrewLinks\
							{\
								class LinkIDProvider\
								{\
									nextID=1;\
								};\
								class Links\
								{\
									items=1;\
									class Item0\
									{\
										linkID=0;\
										item0=275;\
										item1=273;\
										class CustomData\
										{\
											role=2;\
											turretPath[]={0};\
										};\
									};\
								};\
							};\
							id=283;\
						};\
					};\
					id=258;\
				};\
			};\
			id=250;\
			atlOffset=4.5767746;\
		};\
		class Item15\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={9530.4922,127.11944,13270.403};\
				angles[]={0.0024958209,0,0.10213667};\
			};\
			areaSize[]={5,-1,5};\
			flags=1;\
			id=271;\
			type=\"ModuleHideTerrainObjects_F\";\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"#filter\";\
					expression=\"_this setVariable [\"\"#filter\"\",_value]\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=15;\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"#hideLocally\";\
					expression=\"_this setVariable [\"\"#hideLocally\"\",_value]\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				nAttributes=2;\
			};\
		};\
		class Item16\
		{\
			dataType=\"Layer\";\
			name=\"HQ\";\
			class Entities\
			{\
				items=4;\
				class Item0\
				{\
					dataType=\"Layer\";\
					name=\"Empty\";\
					class Entities\
					{\
						items=115;\
						class Item0\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9542.583,78.75972,13729.356};\
								angles[]={0.047463283,0,6.2731848};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								disableSimulation=1;\
							};\
							id=553;\
							type=\"Land_HelipadCircle_F\";\
							atlOffset=-1.5258789e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item1\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9565.9219,78.778351,13726.365};\
								angles[]={6.2207661,0,0.057437535};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								disableSimulation=1;\
							};\
							id=564;\
							type=\"Land_HelipadCircle_F\";\
							atlOffset=1.5258789e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item2\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9563.2441,81.865219,13724.85};\
								angles[]={6.2532005,0.77147043,0.037472218};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								ammo=0.17383911;\
								fuel=0;\
								disableSimulation=1;\
							};\
							id=567;\
							type=\"RHS_Mi8MTV3_heavy_vdv\";\
							atlOffset=-0.016136169;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute2\
								{\
									property=\"rhs_decalNumber_type\";\
									expression=\"if(_value != 'NoChange')then{ _this setVariable ['rhs_decalNumber_type', _value];[_this,[['Number', cRHSAIRMI8NumberPlaces, _value]]] call rhs_fnc_decalsInit}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"NoChange\";\
										};\
									};\
								};\
								class Attribute3\
								{\
									property=\"ammoBox\";\
									expression=\"[_this,_value] call bis_fnc_initAmmoBox;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"[[[[\"\"FirstAidKit\"\",\"\"Medikit\"\"],[8,1]],[[\"\"rhs_30Rnd_545x39_7N10_AK\"\",\"\"rhs_mag_rgd5\"\",\"\"rhs_mag_nspn_red\"\",\"\"rhs_mag_rdg2_white\"\"],[30,10,10,4]],[[\"\"ToolKit\"\"],[1]],[[\"\"rhs_d6_Parachute_backpack\"\"],[8]]],false]\";\
										};\
									};\
								};\
								nAttributes=4;\
							};\
						};\
						class Item3\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9542.3047,81.988678,13728.352};\
								angles[]={0.047463283,0.012946546,6.2731848};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=568;\
							type=\"RHS_Mi24P_vdv\";\
							atlOffset=-4.5776367e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"rhs_decalNumber_type\";\
									expression=\"if(_value != 'NoChange')then{ _this setVariable ['rhs_decalNumber_type', _value];[_this,[['Number', cRHSAIRMI24NumberPlaces, _value]]] call rhs_fnc_decalsInit}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"NoChange\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item4\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9670.3789,94.039894,13589.252};\
								angles[]={0,1.0935346,0};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=583;\
							type=\"CamoNet_BLUFOR_open_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item5\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9619.7939,82.623993,13768.04};\
								angles[]={6.2240763,4.8028173,6.219099};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=589;\
							type=\"Land_HBarrierTower_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item6\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9596.9531,92.984459,13841.59};\
								angles[]={6.2108116,4.2367353,6.1416383};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=590;\
							type=\"Land_HBarrierTower_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item7\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9907.7832,-3.806581,13843.053};\
								angles[]={0,2.434098,0};\
							};\
							side=\"Empty\";\
							flags=1;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=598;\
							type=\"Land_Pier_small_F\";\
							atlOffset=7.1845264;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item8\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9916.7148,3.3238735,13838.612};\
								angles[]={0,2.6417103,0};\
							};\
							side=\"Empty\";\
							class Attributes\
							{\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=597;\
							type=\"O_Boat_Armed_01_hmg_F\";\
							atlOffset=7.4482665;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item9\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9618.958,82.187012,13775.528};\
								angles[]={6.1294084,1.5703669,6.1245308};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=599;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item10\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9617.7461,83.808586,13783.314};\
								angles[]={6.0978374,1.3704138,6.1318517};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=600;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item11\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9615.6348,85.382637,13791.12};\
								angles[]={6.1294084,1.3373963,6.1859927};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=601;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item12\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9604.1846,89.724472,13827.883};\
								angles[]={6.2332287,1.1547222,6.1588306};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=604;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item13\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9600.4521,90.624275,13835.056};\
								angles[]={6.2157874,1.1217047,6.1612935};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=605;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item14\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9609.3213,88.200684,13812.314};\
								angles[]={6.2108116,1.3134801,6.1884708};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=606;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item15\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9613.2949,86.660103,13799.135};\
								angles[]={6.1711569,1.3134801,6.1785693};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=607;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item16\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9611.2715,87.509521,13805.615};\
								angles[]={6.1959076,1.265654,6.2033553};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=602;\
							type=\"Land_HBarrierWall4_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item17\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9615,82.233566,13750.154};\
								angles[]={0.079830162,2.0274847,6.1810431};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=609;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item18\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9618.3643,81.448456,13761.998};\
								angles[]={0.015002378,1.6830777,6.1026711};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=613;\
							type=\"Land_HBarrierWall4_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item19\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9617.3535,81.667427,13756.749};\
								angles[]={0.0050033992,1.9101497,6.1810412};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=612;\
							type=\"Land_HBarrierWall4_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item20\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9611.9063,82.664268,13744.313};\
								angles[]={0.029993678,2.1771603,6.2058411};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=611;\
							type=\"Land_HBarrierWall_corner_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item21\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9593.041,92.654457,13847.707};\
								angles[]={6.2083244,0.95144588,6.1147971};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=614;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item22\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9587.5371,94.06102,13853.346};\
								angles[]={6.2058401,0.7514928,6.105092};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=615;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item23\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9581.2881,95.575111,13858.479};\
								angles[]={6.1983881,0.71847528,6.1147952};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=616;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item24\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9556.3145,99.13031,13858.986};\
								angles[]={6.1637573,5.7155123,6.183517};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=617;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item25\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9549.9639,99.04377,13854.057};\
								angles[]={6.1489964,5.5567541,6.2083244};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=618;\
							type=\"Land_HBarrierWall6_F\";\
							atlOffset=7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item26\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9544.2734,98.490265,13848.312};\
								angles[]={6.1245308,5.5237365,6.2357197};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=619;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item27\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9563.2197,98.82515,13863.703};\
								angles[]={6.1835165,5.7155123,6.1539092};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=620;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item28\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9574.7334,97.116272,13863.649};\
								angles[]={6.1983881,0.69455916,6.1245303};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=621;\
							type=\"Land_HBarrierWall6_F\";\
							atlOffset=7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item29\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9569.4219,99.637543,13866.74};\
								angles[]={6.1983886,3.3833766,6.1294103};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=623;\
							type=\"Land_HBarrierTower_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item30\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9535.1357,96.444733,13835.001};\
								angles[]={6.1075153,5.2921901,6.2731848};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=624;\
							type=\"Land_HBarrierWall6_F\";\
							atlOffset=1.5258789e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item31\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9531.3711,95.161316,13827.897};\
								angles[]={6.1002545,5.1334319,0.0049914722};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=625;\
							type=\"Land_HBarrierWall6_F\";\
							atlOffset=2.2888184e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item32\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9528.543,93.711243,13820.321};\
								angles[]={6.0954232,5.1004143,0.012501417};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=626;\
							type=\"Land_HBarrierWall6_F\";\
							atlOffset=7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item33\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9539.4941,97.634804,13842.138};\
								angles[]={6.1123662,5.2921901,6.2581916};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=627;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item34\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9523.1035,90.694023,13804.741};\
								angles[]={6.1026711,5.0515671,0.027494613};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=628;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item35\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9520.6445,89.264671,13797.055};\
								angles[]={6.1123676,5.0156741,0.029989703};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=629;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item36\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9518.3262,87.84169,13789.141};\
								angles[]={6.1172266,5.0360184,0.03748076};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=630;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item37\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9525.6348,92.215446,13812.712};\
								angles[]={6.0954232,5.0515671,0.022498533};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=631;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item38\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9516.4922,85.102272,13773.1};\
								angles[]={6.105094,4.804472,0.042473838};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=632;\
							type=\"Land_HBarrierWall6_F\";\
							atlOffset=1.5258789e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item39\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9516.082,83.252541,13765.001};\
								angles[]={6.0429173,4.7373018,0.047465794};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=633;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item40\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9516.3965,81.494057,13756.92};\
								angles[]={6.0954232,4.7042847,0.039977662};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=634;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item41\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9516.9971,86.532089,13781.447};\
								angles[]={6.119658,4.804472,0.039980642};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=635;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item42\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9517.0186,81.947235,13749.524};\
								angles[]={6.166223,1.5000929,0.032487731};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=636;\
							type=\"Land_HBarrierTower_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item43\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9518.0557,79.574173,13742.196};\
								angles[]={6.1367421,4.7373018,0.032487731};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=637;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item44\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9519.3164,78.503342,13734.653};\
								angles[]={6.1810436,4.364922,0.11942752};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=638;\
							type=\"Land_HBarrierWall6_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item45\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9607.3525,89.036232,13822.345};\
								angles[]={6.2307339,2.7416961,6.173625};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=642;\
							type=\"Land_HBarrierWall_corridor_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item46\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9602.5283,89.363899,13820.078};\
								angles[]={6.2083263,2.7416961,6.1909471};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=644;\
							type=\"Land_HBarrierWall_corridor_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item47\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9600.3096,89.164177,13815.052};\
								angles[]={6.1959076,4.3448753,6.2033539};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=646;\
							type=\"Land_HBarrierWall_corridor_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item48\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9607.7256,88.718018,13817.473};\
								angles[]={6.220767,1.2831665,6.1884708};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=643;\
							type=\"Land_HBarrierWall4_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item49\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9544.627,93.072128,13792.468};\
								angles[]={6.1247487,1.5126644,0};\
							};\
							side=\"Empty\";\
							flags=1;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=661;\
							type=\"Land_TentHangar_V1_F\";\
							atlOffset=0.70361328;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item50\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9545.6592,97.242073,13817.215};\
								angles[]={6.1226263,1.5967623,0};\
							};\
							side=\"Empty\";\
							flags=1;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=662;\
							type=\"Land_TentHangar_V1_F\";\
							atlOffset=0.70365143;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item51\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9549.1396,89.825569,13798.4};\
								angles[]={6.1245303,1.5559527,0.0075011365};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=678;\
							type=\"CargoNet_01_barrels_F\";\
							atlOffset=-0.0026092529;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item52\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9549.334,90.420189,13800.209};\
								angles[]={6.1220932,0,0.0074931863};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=680;\
							type=\"blackorder_CargoNet_01_ammo_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"ammoBox\";\
									expression=\"[_this,_value] call bis_fnc_initAmmoBox;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"[[[[],[]],[[],[]],[[],[]],[[],[]]],false]\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item53\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9546.9541,90.52549,13800.967};\
								angles[]={6.1220932,0.7640714,0.0074931863};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=681;\
							type=\"blackorder_CargoNet_01_ammo_F\";\
							atlOffset=3.8146973e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"ammoBox\";\
									expression=\"[_this,_value] call bis_fnc_initAmmoBox;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"[[[[],[]],[[],[]],[[],[]],[[],[]]],false]\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item54\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9542.0625,88.732681,13789.081};\
								angles[]={6.1269674,3.4364233,0.019999012};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=677;\
							type=\"Land_EngineCrane_01_F\";\
							atlOffset=3.8146973e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item55\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9544.083,91.569679,13797.715};\
								angles[]={6.1220932,4.7174368,0.014998405};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=679;\
							type=\"Land_RepairDepot_01_green_F\";\
							atlOffset=-0.00019836426;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item56\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9546.3857,91.155746,13792.663};\
								angles[]={6.1220913,1.503636,0.020001473};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								name=\"t_14_1\";\
								textures=\"Grey\";\
								disableSimulation=1;\
								reportRemoteTargets=1;\
								receiveRemoteTargets=1;\
								reportOwnPosition=1;\
							};\
							id=669;\
							type=\"O_MBT_04_command_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"VehicleCustomization\";\
									expression=\"if (local _this) then {if (isSimpleObject _this) then {_this setVariable ['bis_fnc_initVehicle_customization',_value]} else {([_this] + _value + [true]) call (uinamespace getvariable 'bis_fnc_initVehicle');};};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"ARRAY\"\
												};\
											};\
											class value\
											{\
												items=2;\
												class Item0\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"ARRAY\"\
															};\
														};\
													};\
												};\
												class Item1\
												{\
													class data\
													{\
														class type\
														{\
															type[]=\
															{\
																\"ARRAY\"\
															};\
														};\
														class value\
														{\
															items=4;\
															class Item0\
															{\
																class data\
																{\
																	class type\
																	{\
																		type[]=\
																		{\
																			\"STRING\"\
																		};\
																	};\
																	value=\"showCamonetHull\";\
																};\
															};\
															class Item1\
															{\
																class data\
																{\
																	class type\
																	{\
																		type[]=\
																		{\
																			\"SCALAR\"\
																		};\
																	};\
																	value=1;\
																};\
															};\
															class Item2\
															{\
																class data\
																{\
																	class type\
																	{\
																		type[]=\
																		{\
																			\"STRING\"\
																		};\
																	};\
																	value=\"showCamonetTurret\";\
																};\
															};\
															class Item3\
															{\
																class data\
																{\
																	class type\
																	{\
																		type[]=\
																		{\
																			\"SCALAR\"\
																		};\
																	};\
																	value=1;\
																};\
															};\
														};\
													};\
												};\
											};\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute2\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute3\
								{\
									property=\"Enh_damageLTrack\";\
									expression=\"_this setHitPointDamage ['hitLTrack',_value]\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"SCALAR\"\
												};\
											};\
											value=1;\
										};\
									};\
								};\
								class Attribute4\
								{\
									property=\"ammoBox\";\
									expression=\"[_this,_value] call bis_fnc_initAmmoBox;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"[[[[],[]],[[],[]],[[],[]],[[],[]]],false]\";\
										};\
									};\
								};\
								nAttributes=5;\
							};\
						};\
						class Item57\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9544.6445,88.306274,13789.41};\
								angles[]={6.1294084,5.4387684,0.01749678};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=676;\
							type=\"Land_TankTracks_01_long_F\";\
							atlOffset=-2.2888184e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item58\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9549.585,97.26265,13817.478};\
								angles[]={6.1226263,1.5967623,0};\
							};\
							side=\"Empty\";\
							flags=1;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=705;\
							type=\"Land_TentHangar_V1_F\";\
							atlOffset=0.70365143;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item59\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9595.9053,89.368027,13812.516};\
								angles[]={6.1909471,2.7357631,6.1736274};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=740;\
							type=\"Land_HBarrierWall4_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item60\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9590.9854,89.634361,13810.146};\
								angles[]={6.1785693,2.7357631,6.1760964};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=741;\
							type=\"Land_HBarrierWall4_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item61\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9586.0244,89.933792,13808.303};\
								angles[]={6.1637573,2.9558105,6.183517};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=742;\
							type=\"Land_HBarrierWall4_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item62\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9580.792,90.304016,13807.191};\
								angles[]={6.1563692,3.0118167,6.1959076};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=743;\
							type=\"Land_HBarrierWall_corner_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item63\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9567.4316,91.105331,13806.57};\
								angles[]={6.1391902,3.134207,6.2432051};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=744;\
							type=\"Land_HBarrierWall4_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item64\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9562.127,91.294136,13806.618};\
								angles[]={6.1342955,3.2636044,6.2556911};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=745;\
							type=\"Land_HBarrierWall4_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item65\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9533.082,90.994667,13805.362};\
								angles[]={6.1050925,0,0.017500184};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=726;\
							type=\"Land_HBarrier_5_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"hideObject\";\
									expression=\"if !(is3DEN) then {_this hideobjectglobal _value;};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=1;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item66\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9538.9092,91.065216,13805.302};\
								angles[]={6.1075153,0,0.014998405};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=727;\
							type=\"Land_HBarrier_5_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"hideObject\";\
									expression=\"if !(is3DEN) then {_this hideobjectglobal _value;};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=1;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item67\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9544.6973,91.136192,13805.336};\
								angles[]={6.1123676,0,0.0075011365};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=728;\
							type=\"Land_HBarrier_5_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"hideObject\";\
									expression=\"if !(is3DEN) then {_this hideobjectglobal _value;};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=1;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item68\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9550.5244,91.156776,13805.275};\
								angles[]={6.1172266,0,0.0050033992};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=729;\
							type=\"Land_HBarrier_5_F\";\
							atlOffset=7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"hideObject\";\
									expression=\"if !(is3DEN) then {_this hideobjectglobal _value;};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=1;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item69\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9556.2559,91.167801,13805.374};\
								angles[]={6.1245308,0,6.2731786};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=730;\
							type=\"Land_HBarrier_5_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"hideObject\";\
									expression=\"if !(is3DEN) then {_this hideobjectglobal _value;};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=1;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item70\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9571,90.784187,13805.723};\
								angles[]={6.1440897,1.5450314,6.2307339};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=735;\
							type=\"Land_HBarrier_5_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"hideObject\";\
									expression=\"if !(is3DEN) then {_this hideobjectglobal _value;};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=1;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item71\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9571.0459,92.035728,13805.528};\
								angles[]={6.1440897,1.5450314,6.2307339};\
							};\
							side=\"Empty\";\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=746;\
							type=\"Land_HBarrier_5_F\";\
							atlOffset=1.2811584;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"hideObject\";\
									expression=\"if !(is3DEN) then {_this hideobjectglobal _value;};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=1;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item72\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9597.3789,89.81739,13819.279};\
								angles[]={6.2033563,2.0426135,6.1785693};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=747;\
							type=\"Land_HBarrierWall4_F\";\
							atlOffset=-7.6293945e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"hideObject\";\
									expression=\"if !(is3DEN) then {_this hideobjectglobal _value;};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=1;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item73\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9537.9824,94.211647,13821.494};\
								angles[]={6.1002574,4.8470764,0};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=725;\
							type=\"Land_MapBoard_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute2\
								{\
									property=\"hideObject\";\
									expression=\"if !(is3DEN) then {_this hideobjectglobal _value;};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=3;\
							};\
						};\
						class Item74\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9551.8408,91.60228,13810.752};\
								angles[]={6.1220951,5.3648987,6.278182};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=999;\
							type=\"Box_NATO_Ammo_F\";\
							atlOffset=-0.0035400391;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute2\
								{\
									property=\"ammoBox\";\
									expression=\"[_this,_value] call bis_fnc_initAmmoBox;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"[[[[],[]],[[],[]],[[],[]],[[],[]]],false]\";\
										};\
									};\
								};\
								nAttributes=3;\
							};\
						};\
						class Item75\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9552.4492,91.709106,13809.441};\
								angles[]={6.1220951,3.0537682,6.278182};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								disableSimulation=1;\
							};\
							id=997;\
							type=\"Land_PaperBox_open_empty_F\";\
							atlOffset=8.392334e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item76\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9557.3076,91.597397,13811.09};\
								angles[]={6.1294084,3.2704511,6.2631893};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=1009;\
							type=\"Land_CanisterFuel_F\";\
							atlOffset=-0.0022201538;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item77\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9549.1699,92.06736,13810.542};\
								angles[]={6.1147952,3.0314107,0.0025135824};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								disableSimulation=1;\
							};\
							id=1002;\
							type=\"Land_CratesShabby_F\";\
							atlOffset=0.012458801;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item78\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9556.4424,91.723663,13808.686};\
								angles[]={6.1245308,3.2705035,6.268187};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								disableSimulation=1;\
							};\
							id=1003;\
							type=\"Land_CratesShabby_F\";\
							atlOffset=0.010772705;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item79\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9556.1924,91.933128,13810.104};\
								angles[]={6.1245308,6.0777264,6.268187};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								disableSimulation=1;\
							};\
							id=994;\
							type=\"Land_CratesWooden_F\";\
							atlOffset=0.010452271;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item80\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9550.3086,91.572029,13810.519};\
								angles[]={6.1220951,5.1030989,6.278182};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=1000;\
							type=\"Box_NATO_AmmoOrd_F\";\
							atlOffset=-0.0035247803;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute2\
								{\
									property=\"ammoBox\";\
									expression=\"[_this,_value] call bis_fnc_initAmmoBox;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"[[[[],[]],[[],[]],[[],[]],[[],[]]],false]\";\
										};\
									};\
								};\
								nAttributes=3;\
							};\
						};\
						class Item81\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9554.665,91.497726,13811.739};\
								angles[]={6.1245303,3.2705035,6.2756844};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								disableSimulation=1;\
							};\
							id=991;\
							type=\"Land_Garbage_square5_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item82\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9550.8623,91.342049,13810.627};\
								angles[]={6.1220951,3.2705035,6.278182};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								disableSimulation=1;\
							};\
							id=993;\
							type=\"Land_Garbage_square5_F\";\
							atlOffset=4.5776367e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item83\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9556.2979,91.642563,13811.199};\
								angles[]={6.1245308,1.4379077,6.268187};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=998;\
							type=\"Box_NATO_Grenades_F\";\
							atlOffset=-0.0025405884;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute2\
								{\
									property=\"ammoBox\";\
									expression=\"[_this,_value] call bis_fnc_initAmmoBox;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"[[[[],[]],[[],[]],[[],[]],[[],[]]],false]\";\
										};\
									};\
								};\
								nAttributes=3;\
							};\
						};\
						class Item84\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9548.4121,91.818459,13811.741};\
								angles[]={6.1220951,4.4975872,6.278182};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								disableSimulation=1;\
							};\
							id=995;\
							type=\"Land_Pallets_F\";\
							atlOffset=0.0042419434;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item85\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9555.6572,91.097733,13808.935};\
								angles[]={6.1245303,1.6497555,6.2756844};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=1014;\
							type=\"Land_RiceBox_F\";\
							atlOffset=-0.0010375977;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item86\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9555.5166,91.088951,13808.873};\
								angles[]={6.1245303,3.2709575,6.2756844};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=1015;\
							type=\"Land_RiceBox_F\";\
							atlOffset=-0.0010223389;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item87\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9557.29,91.418053,13809.154};\
								angles[]={6.1245308,4.114543,6.268187};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								disableSimulation=1;\
							};\
							id=1007;\
							type=\"Land_Sack_F\";\
							atlOffset=0.0054473877;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item88\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9557.0332,90.986855,13808.328};\
								angles[]={6.1245308,0.32224083,6.268187};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=1010;\
							type=\"Land_TinContainer_F\";\
							atlOffset=-0.00079345703;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item89\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9548.7832,91.091515,13808.914};\
								angles[]={6.1147952,3.2706146,0.0025135824};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=1008;\
							type=\"Land_Tyre_F\";\
							atlOffset=-0.0012512207;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item90\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9555.9814,91.13327,13808.949};\
								angles[]={6.1245303,3.2706578,6.2756844};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=1011;\
							type=\"Land_BottlePlastic_V2_F\";\
							atlOffset=-0.0012359619;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item91\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9555.8887,91.122559,13808.878};\
								angles[]={6.1245303,1.9103684,6.2756844};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=1012;\
							type=\"Land_BottlePlastic_V2_F\";\
							atlOffset=-0.0012359619;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item92\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9556.04,91.116257,13808.847};\
								angles[]={6.1245308,3.2706578,6.268187};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=1013;\
							type=\"Land_BottlePlastic_V2_F\";\
							atlOffset=-0.0011062622;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item93\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9550.3809,91.12487,13809.535};\
								angles[]={6.1147952,6.1464748,0.0025135824};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=1005;\
							type=\"Land_WoodenBox_F\";\
							atlOffset=-0.0019989014;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item94\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9550.6221,90.990288,13808.74};\
								angles[]={6.1147952,6.1270456,0.0025135824};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								skill=0.2;\
								createAsSimpleObject=1;\
								disableSimulation=1;\
							};\
							id=1006;\
							type=\"Land_WoodenBox_F\";\
							atlOffset=-0.0020599365;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item95\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9546.0146,95.838554,13832.995};\
								angles[]={0,4.729063,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=1023;\
							type=\"Land_Cargo_House_V1_F\";\
							atlOffset=-0.00025177002;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item96\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9548.6582,96.736481,13838.906};\
								angles[]={0,5.4942613,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=1028;\
							type=\"Land_Cargo_House_V1_F\";\
							atlOffset=-7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item97\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9554.832,96.808945,13841.281};\
								angles[]={0,0.0022671742,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=1029;\
							type=\"Land_Cargo_House_V1_F\";\
							atlOffset=-0.00011444092;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item98\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9586.707,93.403564,13844.65};\
								angles[]={0,5.7775373,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=1031;\
							type=\"Land_Medevac_house_V1_F\";\
							atlOffset=6.8664551e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item99\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9575.7393,99.266098,13848.794};\
								angles[]={0,5.383502,0};\
							};\
							side=\"Empty\";\
							flags=1;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=1030;\
							type=\"Land_Cargo_HQ_V1_F\";\
							atlOffset=0.52815247;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item100\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9608.9131,86.930611,13751.331};\
								angles[]={0,5.7622809,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=1035;\
							type=\"Land_Cargo_Patrol_V1_F\";\
							atlOffset=9.9182129e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item101\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9606.5596,82.415741,13745.721};\
								angles[]={6.223259,4.1362247,6.2531919};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=1036;\
							type=\"Land_BagFence_Long_F\";\
							atlOffset=-2.2888184e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item102\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9604.9434,82.609573,13748.312};\
								angles[]={6.255693,4.1362247,6.2531939};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=1037;\
							type=\"Land_BagFence_Long_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item103\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9603.335,82.708824,13750.892};\
								angles[]={6.255693,4.1362247,0};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=1038;\
							type=\"Land_BagFence_Long_F\";\
							atlOffset=-7.6293945e-006;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item104\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9604.4805,82.645027,13755.452};\
								angles[]={0.019999012,5.713645,6.2307329};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=1040;\
							type=\"Land_BagFence_Long_F\";\
							atlOffset=-1.5258789e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item105\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9602.4805,82.728325,13753.465};\
								angles[]={0.0075011365,1.8136258,0};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								disableSimulation=1;\
							};\
							id=1039;\
							type=\"Land_BagFence_Round_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item106\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9605.8477,83.251442,13748.475};\
								angles[]={0,1.0474768,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								init=\"call{this switchLight \"\"ON\"\"}\";\
								disableSimulation=1;\
							};\
							id=1056;\
							type=\"Land_PortableLight_double_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item107\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9603.4189,83.392624,13752.923};\
								angles[]={0,1.6659867,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								init=\"call{this switchLight \"\"ON\"\"}\";\
								disableSimulation=1;\
							};\
							id=1057;\
							type=\"Land_PortableLight_double_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item108\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9570.4756,90.591866,13801.794};\
								angles[]={0,5.4934139,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								init=\"call{this switchLight \"\"ON\"\"}\";\
								disableSimulation=1;\
							};\
							id=1058;\
							type=\"Land_PortableLight_double_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item109\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9557.3018,94.808456,13826.308};\
								angles[]={0,0.99187678,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								init=\"call{this switchLight \"\"ON\"\"}\";\
								disableSimulation=1;\
							};\
							id=1059;\
							type=\"Land_PortableLight_double_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item110\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9539.2656,92.396011,13810.854};\
								angles[]={0,4.3177886,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								init=\"call{this switchLight \"\"ON\"\"}\";\
								disableSimulation=1;\
							};\
							id=1060;\
							type=\"Land_PortableLight_double_F\";\
							atlOffset=0.00016784668;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item111\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9537.4697,87.913475,13784.082};\
								angles[]={0,3.9277792,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								init=\"call{this switchLight \"\"ON\"\"}\";\
								disableSimulation=1;\
							};\
							id=1061;\
							type=\"Land_PortableLight_double_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item112\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9553.4873,88.361588,13785.134};\
								angles[]={0,2.3218012,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								init=\"call{this switchLight \"\"ON\"\"}\";\
								disableSimulation=1;\
							};\
							id=1062;\
							type=\"Land_PortableLight_double_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item113\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9552.3125,90.968971,13801.97};\
								angles[]={0,0.51505506,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								init=\"call{this switchLight \"\"ON\"\"}\";\
								disableSimulation=1;\
							};\
							id=1063;\
							type=\"Land_PortableLight_double_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item114\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9536.583,90.556389,13800.544};\
								angles[]={0,5.823184,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
								init=\"call{this switchLight \"\"ON\"\"}\";\
								disableSimulation=1;\
							};\
							id=1064;\
							type=\"Land_PortableLight_double_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
					};\
					id=563;\
					atlOffset=-0.0080032349;\
				};\
				class Item1\
				{\
					dataType=\"Layer\";\
					name=\"REDFOR\";\
					class Entities\
					{\
						items=13;\
						class Item0\
						{\
							dataType=\"Group\";\
							side=\"East\";\
							class Entities\
							{\
								items=2;\
								class Item0\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9672.7256,92.873108,13586.407};\
										angles[]={0,1.8973823,0};\
									};\
									side=\"East\";\
									class Attributes\
									{\
									};\
									id=579;\
									type=\"rhs_msv_rifleman\";\
									atlOffset=9.1010742;\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"speaker\";\
											expression=\"_this setspeaker _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"RHS_Male01RUS\";\
												};\
											};\
										};\
										class Attribute2\
										{\
											property=\"pitch\";\
											expression=\"_this setpitch _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"SCALAR\"\
														};\
													};\
													value=0.99000001;\
												};\
											};\
										};\
										class Attribute3\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=4;\
									};\
								};\
								class Item1\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9672.7256,92.873108,13586.407};\
										angles[]={0,1.8973823,0};\
									};\
									side=\"East\";\
									flags=2;\
									class Attributes\
									{\
									};\
									id=580;\
									type=\"rhs_msv_sergeant\";\
									atlOffset=9.1010742;\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"speaker\";\
											expression=\"_this setspeaker _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"RHS_Male01RUS\";\
												};\
											};\
										};\
										class Attribute2\
										{\
											property=\"pitch\";\
											expression=\"_this setpitch _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"SCALAR\"\
														};\
													};\
													value=1.01;\
												};\
											};\
										};\
										class Attribute3\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										class Attribute4\
										{\
											property=\"face\";\
											expression=\"_this setface _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"\";\
												};\
											};\
										};\
										nAttributes=5;\
									};\
								};\
							};\
							class Attributes\
							{\
							};\
							class CrewLinks\
							{\
								class LinkIDProvider\
								{\
									nextID=2;\
								};\
								class Links\
								{\
									items=2;\
									class Item0\
									{\
										linkID=0;\
										item0=579;\
										item1=578;\
										class CustomData\
										{\
											role=2;\
											turretPath[]={0};\
										};\
									};\
									class Item1\
									{\
										linkID=1;\
										item0=580;\
										item1=578;\
										class CustomData\
										{\
											role=2;\
											turretPath[]={1};\
										};\
									};\
								};\
							};\
							id=577;\
							atlOffset=9.0961075;\
						};\
						class Item1\
						{\
							dataType=\"Group\";\
							side=\"East\";\
							class Entities\
							{\
								items=2;\
								class Item0\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9670.9443,92.873497,13592.925};\
										angles[]={0,1.0722615,0};\
									};\
									side=\"East\";\
									flags=4;\
									class Attributes\
									{\
									};\
									id=585;\
									type=\"rhs_msv_rifleman\";\
								};\
								class Item1\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9670.9443,92.873497,13592.925};\
										angles[]={0,1.0722615,0};\
									};\
									side=\"East\";\
									flags=6;\
									class Attributes\
									{\
									};\
									id=586;\
									type=\"rhs_msv_sergeant\";\
								};\
							};\
							class Attributes\
							{\
							};\
							class CrewLinks\
							{\
								class LinkIDProvider\
								{\
									nextID=2;\
								};\
								class Links\
								{\
									items=2;\
									class Item0\
									{\
										linkID=0;\
										item0=585;\
										item1=587;\
										class CustomData\
										{\
											role=2;\
											turretPath[]={0};\
										};\
									};\
									class Item1\
									{\
										linkID=1;\
										item0=586;\
										item1=587;\
										class CustomData\
										{\
											role=2;\
											turretPath[]={1};\
										};\
									};\
								};\
							};\
							id=584;\
						};\
						class Item2\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9672.7207,94.780464,13586.5};\
								angles[]={0,1.8973751,0};\
							};\
							side=\"East\";\
							flags=2;\
							class Attributes\
							{\
							};\
							id=578;\
							type=\"RHS_zu23_MSV\";\
							atlOffset=9.0961075;\
						};\
						class Item3\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9670.9443,94.785805,13592.875};\
								angles[]={0,1.0722615,0};\
							};\
							side=\"East\";\
							flags=6;\
							class Attributes\
							{\
							};\
							id=587;\
							type=\"RHS_zu23_MSV\";\
						};\
						class Item4\
						{\
							dataType=\"Group\";\
							side=\"East\";\
							class Entities\
							{\
								items=1;\
								class Item0\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9598.5488,92.894722,13842.193};\
										angles[]={6.2108064,1.0718416,6.1412725};\
									};\
									side=\"East\";\
									flags=6;\
									class Attributes\
									{\
									};\
									id=656;\
									type=\"rhs_msv_rifleman\";\
									atlOffset=0.00019073486;\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"speaker\";\
											expression=\"_this setspeaker _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"RHS_Male05RUS\";\
												};\
											};\
										};\
										class Attribute2\
										{\
											property=\"pitch\";\
											expression=\"_this setpitch _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"SCALAR\"\
														};\
													};\
													value=0.98000002;\
												};\
											};\
										};\
										class Attribute3\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=4;\
									};\
								};\
							};\
							class Attributes\
							{\
							};\
							class CrewLinks\
							{\
								class LinkIDProvider\
								{\
									nextID=1;\
								};\
								class Links\
								{\
									items=1;\
									class Item0\
									{\
										linkID=0;\
										item0=656;\
										item1=655;\
										class CustomData\
										{\
											role=2;\
											turretPath[]={0};\
										};\
									};\
								};\
							};\
							id=654;\
							atlOffset=2.2876587;\
						};\
						class Item5\
						{\
							dataType=\"Group\";\
							side=\"East\";\
							class Entities\
							{\
								items=1;\
								class Item0\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9621.2031,82.617851,13767.877};\
										angles[]={6.2240767,1.6750658,6.2189875};\
									};\
									side=\"East\";\
									flags=6;\
									class Attributes\
									{\
									};\
									id=659;\
									type=\"rhs_msv_rifleman\";\
								};\
							};\
							class Attributes\
							{\
							};\
							class CrewLinks\
							{\
								class LinkIDProvider\
								{\
									nextID=1;\
								};\
								class Links\
								{\
									items=1;\
									class Item0\
									{\
										linkID=0;\
										item0=659;\
										item1=658;\
										class CustomData\
										{\
											role=2;\
											turretPath[]={0};\
										};\
									};\
								};\
							};\
							id=657;\
							atlOffset=2.4578781;\
						};\
						class Item6\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9598.8037,94.486778,13842.016};\
								angles[]={6.2108126,1.0718393,6.1412716};\
							};\
							side=\"East\";\
							flags=2;\
							class Attributes\
							{\
							};\
							id=655;\
							type=\"rhs_KORD_high_MSV\";\
							atlOffset=2.2876587;\
						};\
						class Item7\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9621.4189,84.216995,13767.715};\
								angles[]={6.2240767,1.6750658,6.2189875};\
							};\
							side=\"East\";\
							flags=2;\
							class Attributes\
							{\
							};\
							id=658;\
							type=\"rhs_KORD_high_MSV\";\
							atlOffset=2.4578781;\
						};\
						class Item8\
						{\
							dataType=\"Group\";\
							side=\"East\";\
							class Entities\
							{\
								items=1;\
								class Item0\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9548.0908,88.373711,13792.693};\
										angles[]={0,0.95434606,0};\
									};\
									side=\"East\";\
									flags=7;\
									class Attributes\
									{\
										init=\"call{if (isServer) then {\" \n \" [[this, \"\"REPAIR_VEH_PRONE\"\", \"\"LIGHT\"\"], BIS_fnc_ambientAnim] remoteExec [\"\"call\"\"]; \" \n \" this disableCollisionWith t_14_1;\" \n \"};}\";\
									};\
									id=683;\
									type=\"rhs_msv_emr_engineer\";\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_disableAI_move\";\
											expression=\"if(_value) then {_this disableAI 'MOVE'}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"BOOL\"\
														};\
													};\
													value=1;\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"allowDamage\";\
											expression=\"_this allowdamage _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"BOOL\"\
														};\
													};\
													value=0;\
												};\
											};\
										};\
										class Attribute2\
										{\
											property=\"face\";\
											expression=\"_this setface _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"\";\
												};\
											};\
										};\
										class Attribute3\
										{\
											property=\"pitch\";\
											expression=\"_this setpitch _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"SCALAR\"\
														};\
													};\
													value=1;\
												};\
											};\
										};\
										class Attribute4\
										{\
											property=\"speaker\";\
											expression=\"_this setspeaker _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"RHS_Male05RUS\";\
												};\
											};\
										};\
										class Attribute5\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										class Attribute6\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										nAttributes=7;\
									};\
								};\
							};\
							class Attributes\
							{\
							};\
							id=682;\
						};\
						class Item9\
						{\
							dataType=\"Group\";\
							side=\"East\";\
							class Entities\
							{\
								items=1;\
								class Item0\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9578.8838,89.14563,13802.977};\
										angles[]={0,2.8881536,0};\
									};\
									side=\"East\";\
									flags=7;\
									class Attributes\
									{\
										init=\"call{if (isServer) then {\" \n \" [[this, \"\"GUARD\"\", \"\"ASIS\"\"], BIS_fnc_ambientAnim] remoteExec [\"\"call\"\"]; \" \n \"};}\";\
									};\
									id=751;\
									type=\"rhs_mvd_izlom_sergeant\";\
									atlOffset=-7.6293945e-006;\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"speaker\";\
											expression=\"_this setspeaker _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"RHS_Male03RUS\";\
												};\
											};\
										};\
										class Attribute2\
										{\
											property=\"pitch\";\
											expression=\"_this setpitch _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"SCALAR\"\
														};\
													};\
													value=0.97000003;\
												};\
											};\
										};\
										class Attribute3\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										class Attribute4\
										{\
											property=\"face\";\
											expression=\"_this setface _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"\";\
												};\
											};\
										};\
										nAttributes=5;\
									};\
								};\
							};\
							class Attributes\
							{\
								behaviour=\"CARELESS\";\
							};\
							id=750;\
							atlOffset=-7.6293945e-006;\
						};\
						class Item10\
						{\
							dataType=\"Group\";\
							side=\"East\";\
							class Entities\
							{\
								items=11;\
								class Item0\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9621.1748,84.125496,13794.828};\
										angles[]={0,5.6767917,0};\
									};\
									side=\"East\";\
									flags=7;\
									class Attributes\
									{\
									};\
									id=754;\
									type=\"rhs_mvd_izlom_sergeant\";\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=2;\
									};\
								};\
								class Item1\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9621.1406,83.828804,13792.171};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
									};\
									id=756;\
									type=\"rhs_mvd_izlom_arifleman\";\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=2;\
									};\
								};\
								class Item2\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9623.3242,83.243202,13792.015};\
										angles[]={0,5.4705267,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
									};\
									id=758;\
									type=\"rhs_mvd_izlom_grenadier_rpg\";\
									atlOffset=7.6293945e-006;\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=2;\
									};\
								};\
								class Item3\
								{\
									dataType=\"Waypoint\";\
									position[]={9605.0889,89.793228,13845.193};\
									class Effects\
									{\
									};\
									showWP=\"NEVER\";\
									id=759;\
									type=\"Move\";\
									atlOffset=-2.2888184e-005;\
								};\
								class Item4\
								{\
									dataType=\"Waypoint\";\
									position[]={9573.6094,97.540459,13875.53};\
									class Effects\
									{\
									};\
									showWP=\"NEVER\";\
									id=760;\
									type=\"Move\";\
									atlOffset=-4.5776367e-005;\
								};\
								class Item5\
								{\
									dataType=\"Waypoint\";\
									position[]={9541.5234,100.47926,13864.798};\
									class Effects\
									{\
									};\
									showWP=\"NEVER\";\
									id=761;\
									type=\"Move\";\
									atlOffset=6.1035156e-005;\
								};\
								class Item6\
								{\
									dataType=\"Waypoint\";\
									position[]={9499.0674,83.372826,13774.32};\
									class Effects\
									{\
									};\
									showWP=\"NEVER\";\
									id=762;\
									type=\"Move\";\
									atlOffset=0.00033569336;\
								};\
								class Item7\
								{\
									dataType=\"Waypoint\";\
									position[]={9542.3105,101.88834,13874.188};\
									class Effects\
									{\
									};\
									showWP=\"NEVER\";\
									id=763;\
									type=\"Move\";\
									atlOffset=-0.00029754639;\
								};\
								class Item8\
								{\
									dataType=\"Waypoint\";\
									position[]={9577.8916,97.385269,13882.123};\
									class Effects\
									{\
									};\
									showWP=\"NEVER\";\
									id=764;\
									type=\"Move\";\
									atlOffset=-7.6293945e-006;\
								};\
								class Item9\
								{\
									dataType=\"Waypoint\";\
									position[]={9612.2549,88.529892,13846.429};\
									class Effects\
									{\
									};\
									showWP=\"NEVER\";\
									id=765;\
									type=\"Move\";\
									atlOffset=0.00019836426;\
								};\
								class Item10\
								{\
									dataType=\"Waypoint\";\
									position[]={9622.7695,84.340088,13805.396};\
									type=\"Cycle\";\
									class Effects\
									{\
									};\
									showWP=\"NEVER\";\
									id=766;\
								};\
							};\
							class Attributes\
							{\
								behaviour=\"SAFE\";\
								speedMode=\"LIMITED\";\
								formation=\"COLUMN\";\
							};\
							id=753;\
						};\
						class Item11\
						{\
							dataType=\"Group\";\
							side=\"East\";\
							class Entities\
							{\
								items=1;\
								class Item0\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9608.1436,86.371094,13749.678};\
										angles[]={0,2.7815223,0};\
									};\
									side=\"East\";\
									flags=6;\
									class Attributes\
									{\
									};\
									id=1043;\
									type=\"rhs_msv_rifleman\";\
								};\
							};\
							class Attributes\
							{\
							};\
							class CrewLinks\
							{\
								class LinkIDProvider\
								{\
									nextID=1;\
								};\
								class Links\
								{\
									items=1;\
									class Item0\
									{\
										linkID=0;\
										item0=1043;\
										item1=1042;\
										class CustomData\
										{\
											role=2;\
											turretPath[]={0};\
										};\
									};\
								};\
							};\
							id=1041;\
							atlOffset=0.19823456;\
						};\
						class Item12\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9608.2725,87.982872,13749.584};\
								angles[]={0,2.7815223,0};\
							};\
							side=\"East\";\
							flags=6;\
							class Attributes\
							{\
							};\
							id=1042;\
							type=\"rhs_KORD_high_MSV\";\
							atlOffset=0.19823456;\
						};\
					};\
					id=562;\
					atlOffset=4.2358322;\
				};\
				class Item2\
				{\
					dataType=\"Layer\";\
					name=\"Players\";\
					class Entities\
					{\
						items=3;\
						class Item0\
						{\
							dataType=\"Group\";\
							side=\"East\";\
							class Entities\
							{\
								items=3;\
								class Item0\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9606.7813,82.286446,13733.305};\
										angles[]={0,0.027500471,0};\
									};\
									side=\"East\";\
									flags=7;\
									class Attributes\
									{\
										skill=0.55000001;\
										rank=\"LIEUTENANT\";\
										description=\"Platoon Lead@Lead\";\
										isPlayer=1;\
										isPlayable=1;\
									};\
									id=671;\
									type=\"keko_faction_generic_opfor_lead\";\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"pitch\";\
											expression=\"_this setpitch _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"SCALAR\"\
														};\
													};\
													value=0.95999998;\
												};\
											};\
										};\
										class Attribute2\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=3;\
									};\
								};\
								class Item1\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9570.792,92.131752,13820.6};\
										angles[]={0,1.6111153,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										rank=\"SERGEANT\";\
										description=\"Platoon Sergeant\";\
										isPlayable=1;\
									};\
									id=672;\
									type=\"keko_faction_generic_opfor_sergeant\";\
									atlOffset=0.00015258789;\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=2;\
									};\
								};\
								class Item2\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9571.1045,91.699852,13817.571};\
										angles[]={0,1.3124157,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										rank=\"SERGEANT\";\
										description=\"Platoon Medic\";\
										isPlayable=1;\
									};\
									id=673;\
									type=\"keko_faction_generic_opfor_doctor\";\
									atlOffset=0.00032806396;\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=2;\
									};\
								};\
							};\
							class Attributes\
							{\
							};\
							id=670;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"groupID\";\
									expression=\"[_this, _value] call CBA_fnc_setCallsign\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Lead\";\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
						class Item1\
						{\
							dataType=\"Layer\";\
							name=\"Loadout\";\
							id=936;\
							atlOffset=44.265518;\
						};\
						class Item2\
						{\
							dataType=\"Group\";\
							side=\"East\";\
							class Entities\
							{\
								items=15;\
								class Item0\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9578.2861,91.427223,13820.432};\
										angles[]={0,4.4057522,0};\
									};\
									side=\"East\";\
									flags=7;\
									class Attributes\
									{\
										description=\"Squad Leader@Alpha\";\
										isPlayable=1;\
									};\
									id=883;\
									type=\"keko_faction_generic_opfor_sql\";\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"pitch\";\
											expression=\"_this setpitch _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"SCALAR\"\
														};\
													};\
													value=0.98000002;\
												};\
											};\
										};\
										class Attribute2\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=3;\
									};\
								};\
								class Item1\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9578.0703,91.710754,13822.647};\
										angles[]={0,4.3828773,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"Medic\";\
										isPlayable=1;\
									};\
									id=885;\
									type=\"keko_faction_generic_opfor_medic\";\
									atlOffset=-7.6293945e-006;\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=2;\
									};\
								};\
								class Item2\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9577.7324,92.003342,13824.856};\
										angles[]={0,4.4081626,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"UAV Operator\";\
										isPlayable=1;\
									};\
									id=887;\
									type=\"keko_faction_generic_opfor_uav\";\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=2;\
									};\
								};\
								class Item3\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9580.4004,91.258858,13820.987};\
										angles[]={0,4.4027209,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"Fire-Team Leader\";\
										isPlayable=1;\
									};\
									id=889;\
									type=\"keko_faction_generic_opfor_ftl\";\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=2;\
									};\
								};\
								class Item4\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9582.3662,91.066689,13821.376};\
										angles[]={6.1711555,4.4968853,6.1637573};\
									};\
									side=\"East\";\
									flags=4;\
									class Attributes\
									{\
										description=\"Rifleman\";\
										isPlayable=1;\
									};\
									id=891;\
									type=\"keko_faction_generic_opfor_rifleman_at\";\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"pitch\";\
											expression=\"_this setpitch _value;\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"SCALAR\"\
														};\
													};\
													value=0.99000001;\
												};\
											};\
										};\
										class Attribute2\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=3;\
									};\
								};\
								class Item5\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9584.3545,90.843452,13821.634};\
										angles[]={0,4.5210385,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"Grenadier\";\
										isPlayable=1;\
									};\
									id=893;\
									type=\"keko_faction_generic_opfor_grenadier\";\
								};\
								class Item6\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9586.1914,90.612808,13821.624};\
										angles[]={0,4.5210385,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"Grenadier\";\
										isPlayable=1;\
									};\
									id=895;\
									type=\"keko_faction_generic_opfor_grenadier\";\
								};\
								class Item7\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9580.1758,91.549393,13823.329};\
										angles[]={0,4.4027209,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"Fire-Team Leader\";\
										isPlayable=1;\
									};\
									id=897;\
									type=\"keko_faction_generic_opfor_ftl\";\
									atlOffset=0.00012207031;\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=2;\
									};\
								};\
								class Item8\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9582.168,91.327446,13823.597};\
										angles[]={0,4.4015417,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"LMG\";\
										isPlayable=1;\
									};\
									id=899;\
									type=\"keko_faction_generic_opfor_lmg\";\
								};\
								class Item9\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9583.9707,91.137802,13823.979};\
										angles[]={0,4.4015417,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"LMG\";\
										isPlayable=1;\
									};\
									id=901;\
									type=\"keko_faction_generic_opfor_lmg\";\
									atlOffset=0.00010681152;\
								};\
								class Item10\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9585.4609,90.975143,13824.31};\
										angles[]={0,4.4173708,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"LMG Asst.\";\
										isPlayable=1;\
									};\
									id=903;\
									type=\"keko_faction_generic_opfor_lmg_asst\";\
								};\
								class Item11\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9579.709,91.855003,13825.576};\
										angles[]={0,4.4027209,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"Fire-Team Leader\";\
										isPlayable=1;\
									};\
									id=905;\
									type=\"keko_faction_generic_opfor_ftl\";\
									atlOffset=7.6293945e-006;\
									class CustomAttributes\
									{\
										class Attribute0\
										{\
											property=\"Enh_featureType\";\
											expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"STRING\"\
														};\
													};\
													value=\"0\";\
												};\
											};\
										};\
										class Attribute1\
										{\
											property=\"Enh_ambientAnimParams\";\
											expression=\"	if (is3DEN) then {_this call BIS_fnc_ambientAnim__terminate};	[_this,_value] spawn	{		sleep 0.1;		params ['_unit','_value'];		if ((_value # 0 != '') && !isMultiplayer) then		{			[_unit,_value # 0,_value # 1,objNull,false,false] call BIS_fnc_ambientAnim;			waitUntil {sleep 0.1; ((behaviour _unit) == 'COMBAT') || (damage _unit > 0.6)};			_unit call BIS_fnc_ambientAnim__terminate;		};	}\";\
											class Value\
											{\
												class data\
												{\
													class type\
													{\
														type[]=\
														{\
															\"ARRAY\"\
														};\
													};\
													class value\
													{\
														items=2;\
														class Item0\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"\";\
															};\
														};\
														class Item1\
														{\
															class data\
															{\
																class type\
																{\
																	type[]=\
																	{\
																		\"STRING\"\
																	};\
																};\
																value=\"ASIS\";\
															};\
														};\
													};\
												};\
											};\
										};\
										nAttributes=2;\
									};\
								};\
								class Item12\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9581.6719,91.597389,13825.496};\
										angles[]={0,4.5116158,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"Marksman\";\
										isPlayable=1;\
									};\
									id=907;\
									type=\"keko_faction_generic_opfor_marksman\";\
									atlOffset=2.2888184e-005;\
								};\
								class Item13\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9583.2998,91.450264,13826.107};\
										angles[]={0,4.5116158,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"Marksman\";\
										isPlayable=1;\
									};\
									id=909;\
									type=\"keko_faction_generic_opfor_marksman\";\
								};\
								class Item14\
								{\
									dataType=\"Object\";\
									class PositionInfo\
									{\
										position[]={9584.8564,91.262054,13826.279};\
										angles[]={0,4.4506946,0};\
									};\
									side=\"East\";\
									flags=5;\
									class Attributes\
									{\
										description=\"Spotter\";\
										isPlayable=1;\
									};\
									id=911;\
									type=\"keko_faction_generic_opfor_spotter\";\
								};\
							};\
							class Attributes\
							{\
							};\
							id=882;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"groupID\";\
									expression=\"[_this, _value] call CBA_fnc_setCallsign\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Alpha\";\
										};\
									};\
								};\
								nAttributes=1;\
							};\
						};\
					};\
					id=686;\
					atlOffset=-279.34961;\
				};\
				class Item3\
				{\
					dataType=\"Layer\";\
					name=\"Func\";\
					class Entities\
					{\
						items=45;\
						class Item0\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9575.457,93.520424,13803.527};\
								angles[]={0,6.0854197,0};\
							};\
							side=\"Empty\";\
							flags=5;\
							class Attributes\
							{\
							};\
							id=749;\
							type=\"Land_BarGate_01_open_F\";\
							atlOffset=2.2888184e-005;\
						};\
						class Item1\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9543.8789,94.463554,13825.704};\
								angles[]={6.1099405,1.5802116,6.2731786};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=689;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=-7.6293945e-006;\
						};\
						class Item2\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9543.8828,94.30011,13824.77};\
								angles[]={6.1099405,1.5802116,6.2731786};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=690;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=0.00012969971;\
						};\
						class Item3\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9543.7725,94.13517,13823.825};\
								angles[]={6.1075153,1.5802116,6.2756844};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=691;\
							type=\"Land_CampingChair_V2_F\";\
						};\
						class Item4\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9543.7764,93.969254,13822.891};\
								angles[]={6.1075153,1.5802116,6.2756844};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=692;\
							type=\"Land_CampingChair_V2_F\";\
						};\
						class Item5\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9543.1123,93.311447,13819.172};\
								angles[]={6.1099386,1.5802116,6.2806721};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=693;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=3.8146973e-005;\
						};\
						class Item6\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9543.1162,93.147858,13818.237};\
								angles[]={6.1099386,1.5802116,6.2806721};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=694;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=7.6293945e-006;\
						};\
						class Item7\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9543.0059,92.982872,13817.293};\
								angles[]={6.1099386,1.5802116,6.2806721};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=695;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=7.6293945e-006;\
						};\
						class Item8\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9543.0098,92.816574,13816.358};\
								angles[]={6.105094,1.5802116,0.0025135824};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=696;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=-0.00010681152;\
						};\
						class Item9\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9545.7393,94.442406,13825.714};\
								angles[]={6.1099405,1.5802116,6.2706842};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=697;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=7.6293945e-005;\
						};\
						class Item10\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9545.7432,94.278839,13824.779};\
								angles[]={6.1099405,1.5802116,6.2706842};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=698;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=0.00010681152;\
						};\
						class Item11\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9545.7803,94.124153,13823.897};\
								angles[]={6.1147952,1.5802116,6.2706842};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=699;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=3.8146973e-005;\
						};\
						class Item12\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9545.7842,93.965225,13822.963};\
								angles[]={6.1147952,1.5802116,6.2706842};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=700;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=3.8146973e-005;\
						};\
						class Item13\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9545.1201,93.317986,13819.244};\
								angles[]={6.1123676,1.5802116,6.278182};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=701;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=2.2888184e-005;\
						};\
						class Item14\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9545.124,93.155624,13818.31};\
								angles[]={6.1099386,1.5802116,6.2806721};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=702;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=0.0001449585;\
						};\
						class Item15\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9545.0137,92.990639,13817.365};\
								angles[]={6.1099386,1.5802116,6.2806721};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=703;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=0.0001449585;\
						};\
						class Item16\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9545.0176,92.826981,13816.431};\
								angles[]={6.1099386,1.5802116,6.2806721};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=704;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=4.5776367e-005;\
						};\
						class Item17\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9547.8047,94.446999,13825.967};\
								angles[]={6.1172266,1.5802116,6.2631865};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=706;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=-3.8146973e-005;\
						};\
						class Item18\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9547.7861,94.286446,13825.006};\
								angles[]={6.1172266,1.5802116,6.2631865};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=707;\
							type=\"Land_CampingChair_V2_F\";\
						};\
						class Item19\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9547.6982,94.133392,13824.088};\
								angles[]={6.1099405,1.5802116,6.2706842};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=708;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=9.1552734e-005;\
						};\
						class Item20\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9547.7021,93.973618,13823.153};\
								angles[]={6.1147952,1.5802116,6.2706842};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=709;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=3.0517578e-005;\
						};\
						class Item21\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9547.0381,93.34124,13819.435};\
								angles[]={6.1123676,1.5802116,6.278182};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=710;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=2.2888184e-005;\
						};\
						class Item22\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9547.042,93.180092,13818.5};\
								angles[]={6.1123676,1.5802116,6.278182};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=711;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=0.00010681152;\
						};\
						class Item23\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9546.9316,93.017647,13817.556};\
								angles[]={6.1123676,1.5802116,6.278182};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=712;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=7.6293945e-006;\
						};\
						class Item24\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9546.9355,92.855515,13816.621};\
								angles[]={6.1099386,1.5802116,6.2806721};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=713;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=4.5776367e-005;\
						};\
						class Item25\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9549.8125,94.414375,13826.039};\
								angles[]={6.1172266,1.5802116,6.2606921};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=714;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=-0.00011444092;\
						};\
						class Item26\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9549.667,94.240654,13824.982};\
								angles[]={6.1172266,1.5802116,6.2606921};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=715;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=-0.00012969971;\
						};\
						class Item27\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9549.7061,94.102051,13824.16};\
								angles[]={6.1172266,1.5802116,6.2606921};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=716;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=-0.00012207031;\
						};\
						class Item28\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9549.71,93.947281,13823.226};\
								angles[]={6.119658,1.5802116,6.2606897};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=717;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=0.00012969971;\
						};\
						class Item29\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9549.0459,93.33374,13819.507};\
								angles[]={6.1196599,1.5802116,6.2656889};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=718;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=6.8664551e-005;\
						};\
						class Item30\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9549.0498,93.177208,13818.572};\
								angles[]={6.1123676,1.5802116,6.2731905};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=719;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=3.0517578e-005;\
						};\
						class Item31\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9548.9395,93.015327,13817.628};\
								angles[]={6.1123676,1.5802116,6.2731905};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=720;\
							type=\"Land_CampingChair_V2_F\";\
							atlOffset=-6.1035156e-005;\
						};\
						class Item32\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9548.9434,92.854141,13816.693};\
								angles[]={6.1123676,1.5802116,6.2731905};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=721;\
							type=\"Land_CampingChair_V2_F\";\
						};\
						class Item33\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9573.9854,92.888489,13823.611};\
								angles[]={6.1637526,0,6.1859879};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								init=\"call{[this, true] call ace_arsenal_fnc_initBox;}\";\
								disableSimulation=1;\
							};\
							id=914;\
							type=\"Land_PaperBox_01_open_boxes_F\";\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item34\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9588.124,90.474625,13802.626};\
								angles[]={6.1637559,2.9493966,6.1959085};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=1044;\
							type=\"rhs_tigr_sts_msv\";\
							atlOffset=-0.048240662;\
						};\
						class Item35\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9592.8086,90.084526,13803.816};\
								angles[]={6.1785693,2.8499563,6.1909471};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=1047;\
							type=\"rhs_tigr_m_msv\";\
							atlOffset=-2.2888184e-005;\
						};\
						class Item36\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9597.2021,89.891205,13805.795};\
								angles[]={6.1810412,2.7495747,6.2008734};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=1048;\
							type=\"rhs_tigr_m_msv\";\
						};\
						class Item37\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9566.8008,92.358589,13809.238};\
								angles[]={6.136735,1.5536397,6.2382083};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
								ammo=0;\
								fuel=0;\
								disableSimulation=1;\
							};\
							id=1079;\
							type=\"rhs_gaz66_ammo_vv\";\
							atlOffset=-2.2888184e-005;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"rhs_decalNumber_type\";\
									expression=\"_this setVariable ['rhs_decalNumber_type', _value];[_this,[['Number', cTrucksGaz4NumberPlaces, _value]]] call rhs_fnc_decalsInit\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Default\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"rhs_decalPlatoon_type\";\
									expression=\"_this setVariable ['rhs_decalPlatoon_type', _value];\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Army\";\
										};\
									};\
								};\
								class Attribute2\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								class Attribute3\
								{\
									property=\"ammoBox\";\
									expression=\"[_this,_value] call bis_fnc_initAmmoBox;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"[[[[\"\"FirstAidKit\"\"],[10]],[[\"\"rhs_30Rnd_545x39_AK_green\"\",\"\"rhs_30Rnd_545x39_7N10_AK\"\",\"\"rhs_30Rnd_545x39_7N22_AK\"\",\"\"rhs_100Rnd_762x54mmR\"\",\"\"rhs_100Rnd_762x54mmR_green\"\",\"\"rhs_VOG25\"\",\"\"rhs_VOG25P\"\",\"\"rhs_mag_9x19_17\"\",\"\"rhs_10Rnd_762x54mmR_7N1\"\"],[20,20,20,20,20,100,100,100,100]],[[],[]],[[],[]]],false]\";\
										};\
									};\
								};\
								class Attribute4\
								{\
									property=\"rhs_decalArmy_type\";\
									expression=\"_this setVariable ['rhs_decalArmy_type', _value];\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"Army\";\
										};\
									};\
								};\
								class Attribute5\
								{\
									property=\"rhs_decalArmy\";\
									expression=\"if(parseNumber _value >= 0)then{ [_this, [ [ 'Label', cTrucksGazRightArmyPlaces,  _this getVariable ['rhs_decalArmy_type','Army'],call compile _value] ] ] call rhs_fnc_decalsInit};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"-1\";\
										};\
									};\
								};\
								class Attribute6\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute7\
								{\
									property=\"rhs_decalPlatoon\";\
									expression=\"if(parseNumber _value >= 0)then{ [_this, [ [ 'Label', cTrucksGazRightPlatoonPlaces,  _this getVariable ['rhs_decalPlatoon_type','Army'],call compile _value] ] ] call rhs_fnc_decalsInit};\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"-1\";\
										};\
									};\
								};\
								nAttributes=8;\
							};\
						};\
						class Item38\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9611.9297,87.328209,13789.732};\
								angles[]={6.1367407,5.9911494,6.2033563};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=1049;\
							type=\"rhs_kamaz5350_msv\";\
							atlOffset=-0.0040817261;\
						};\
						class Item39\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9606.5176,87.468384,13787.671};\
								angles[]={6.1440902,5.9911494,6.2108116};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=1050;\
							type=\"rhs_kamaz5350_msv\";\
							atlOffset=0.00012207031;\
						};\
						class Item40\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9601.582,87.534035,13785.635};\
								angles[]={6.1440887,5.9911494,6.2058401};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=1051;\
							type=\"rhs_kamaz5350_msv\";\
							atlOffset=-0.00015258789;\
						};\
						class Item41\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9596.3896,87.777969,13784.333};\
								angles[]={6.1514535,6.0304837,6.1983881};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=1052;\
							type=\"rhs_kamaz5350_open_msv\";\
							atlOffset=8.392334e-005;\
						};\
						class Item42\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9582.9395,89.954315,13802.013};\
								angles[]={6.1588302,2.9744692,6.1959085};\
							};\
							side=\"Empty\";\
							flags=4;\
							class Attributes\
							{\
							};\
							id=1046;\
							type=\"RHS_UAZ_MSV_01\";\
							atlOffset=-8.392334e-005;\
						};\
						class Item43\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9610.0771,87.180077,13748.939};\
								angles[]={0,1.1475539,0};\
							};\
							side=\"Empty\";\
							class Attributes\
							{\
								init=\"call{this switchLight \"\"ON\"\"}\";\
								disableSimulation=1;\
							};\
							id=1066;\
							type=\"Land_FloodLight_F\";\
							atlOffset=5.0630035;\
							class CustomAttributes\
							{\
								class Attribute0\
								{\
									property=\"Enh_featureType\";\
									expression=\"if (_value isEqualType 0) then {_this setFeatureType _value}; if (_value isEqualType 'STRING') then {_this setFeatureType parseNumber _value}\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"STRING\"\
												};\
											};\
											value=\"0\";\
										};\
									};\
								};\
								class Attribute1\
								{\
									property=\"allowDamage\";\
									expression=\"_this allowdamage _value;\";\
									class Value\
									{\
										class data\
										{\
											class type\
											{\
												type[]=\
												{\
													\"BOOL\"\
												};\
											};\
											value=0;\
										};\
									};\
								};\
								nAttributes=2;\
							};\
						};\
						class Item44\
						{\
							dataType=\"Object\";\
							class PositionInfo\
							{\
								position[]={9610.0596,87.047066,13748.634};\
								angles[]={0,2.6858063,0};\
							};\
							side=\"Empty\";\
							flags=1;\
							class Attributes\
							{\
							};\
							id=1067;\
							type=\"Reflector_Cone_01_Long_white_F\";\
							atlOffset=5.0895157;\
						};\
					};\
					id=912;\
					atlOffset=4.8631058;\
				};\
			};\
			id=400;\
			atlOffset=1.3037033;\
		};\
		class Item17\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={9519.7598,77.699173,13733.737};\
			};\
			areaSize[]={5,-1,5};\
			flags=1;\
			id=639;\
			type=\"ModuleHideTerrainObjects_F\";\
			atlOffset=-6.8664551e-005;\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"#filter\";\
					expression=\"_this setVariable [\"\"#filter\"\",_value]\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=15;\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"#hideLocally\";\
					expression=\"_this setVariable [\"\"#hideLocally\"\",_value]\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				nAttributes=2;\
			};\
		};\
		class Item18\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={9632.8389,77.589386,13749.147};\
				angles[]={0.019996032,0.26780331,5.7746019};\
			};\
			areaSize[]={10,0,30};\
			areaIsRectangle=1;\
			flags=1;\
			id=641;\
			type=\"ModuleHideTerrainObjects_F\";\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"#filter\";\
					expression=\"_this setVariable [\"\"#filter\"\",_value]\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=7;\
						};\
					};\
				};\
				class Attribute1\
				{\
					property=\"#hideLocally\";\
					expression=\"_this setVariable [\"\"#hideLocally\"\",_value]\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"BOOL\"\
								};\
							};\
							value=0;\
						};\
					};\
				};\
				nAttributes=2;\
			};\
		};\
		class Item19\
		{\
			dataType=\"Marker\";\
			position[]={9163.1221,78.063004,13431.287};\
			name=\"safezone\";\
			markerType=\"ELLIPSE\";\
			type=\"ellipse\";\
			alpha=0.14842403;\
			fillName=\"Border\";\
			a=1500;\
			b=1500;\
			drawBorder=1;\
			id=1069;\
			atlOffset=0.00019073486;\
		};\
		class Item20\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={8546.2764,77.647263,13282.039};\
			};\
			id=1076;\
			type=\"Ravage_radZone\";\
			class CustomAttributes\
			{\
				class Attribute0\
				{\
					property=\"Ravage_radZone_zoneSize_m\";\
					expression=\"_this setVariable ['zoneSize_m',_value,true];\";\
					class Value\
					{\
						class data\
						{\
							class type\
							{\
								type[]=\
								{\
									\"SCALAR\"\
								};\
							};\
							value=500;\
						};\
					};\
				};\
				nAttributes=1;\
			};\
		};\
		class Item21\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={9565.5049,91.141869,13811.455};\
			};\
			id=1080;\
			type=\"keko_loadout_moduleAddLoadoutMenu3den\";\
			atlOffset=-3.8146973e-005;\
		};\
		class Item22\
		{\
			dataType=\"Logic\";\
			class PositionInfo\
			{\
				position[]={9568.3066,91.000671,13811.369};\
			};\
			id=1081;\
			type=\"keko_logistics_moduleAddLogisticsMenu3den\";\
			atlOffset=-0.00017547607;\
		};\
	};\
	class Connections\
	{\
		class LinkIDProvider\
		{\
			nextID=2;\
		};\
		class Links\
		{\
			items=2;\
			class Item0\
			{\
				linkID=0;\
				item0=1080;\
				item1=1079;\
				class CustomData\
				{\
					type=\"Sync\";\
				};\
			};\
			class Item1\
			{\
				linkID=1;\
				item0=1081;\
				item1=1079;\
				class CustomData\
				{\
					type=\"Sync\";\
				};\
			};\
		};\
	};\
};\
";

        let parsed: Pairs<Rule> = SQMParser::parse(Rule::file, mission_file).unwrap();

        for p in parsed {
            for p2 in p.into_inner() {
                let mut p3 = p2.into_inner();

                let version = p3.next();
                let vnumber = p3.next();

                assert_eq!(version.unwrap().as_span().as_str(), "version");
                assert_eq!(vnumber.unwrap().as_span().as_str(), "53");


                break
            }
        }
    }
}
