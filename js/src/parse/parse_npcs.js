import fs from "fs";
import * as csv from "csv-parse/sync";
import * as list from "../list.js";

export function parse_npcs() {
  const NPCCardList = csv.parse(
    fs.readFileSync("./ffxiv-datamining/csv/TripleTriad.csv").toString(),
    {
      from_line: 4,
      on_record: (record, options) => {
        if (is_zero(record.variable_0) && is_zero(record.fixed_0)) {
          return null;
        }

        const guaranteed = [
          record.fixed_0,
          record.fixed_1,
          record.fixed_2,
          record.fixed_3,
          record.fixed_4,
        ].filter(is_not_zero);

        const variable = [
          record.variable_0,
          record.variable_1,
          record.variable_2,
          record.variable_3,
          record.variable_4,
        ].filter(is_not_zero);

        const rules = [record.rule_0, record.rule_1].filter(is_not_zero);

        const output_record = {
          id: record.id,
          guaranteed,
          variable,
          rules,
        };

        return output_record;
      },
      columns: [
        "id",
        "fixed_0",
        "fixed_1",
        "fixed_2",
        "fixed_3",
        "fixed_4",
        "variable_0",
        "variable_1",
        "variable_2",
        "variable_3",
        "variable_4",
        "rule_0",
        "rule_1",
        "UsesRegionalRules",
        "Fee",
        "PreviousQuestJoin",
        "PreviousQuest[0]",
        "PreviousQuest[1]",
        "PreviousQuest[2]",
        "StartTime",
        "EndTime",
        "DefaultTalk{Challenge}",
        "DefaultTalk{Unavailable}",
        "DefaultTalk{NPCWin}",
        "DefaultTalk{Draw}",
        "DefaultTalk{PCWin}",
        "",
        "Item{PossibleReward}[0]",
        "Item{PossibleReward}[1]",
        "Item{PossibleReward}[2]",
        "Item{PossibleReward}[3]",
      ],
    }
  );

  // list.preview(NPCCardList);
  const NPCCardMap = list.to_map((c) => c.id, NPCCardList);

  const BaseNpcList = csv.parse(
    fs.readFileSync("./ffxiv-datamining/csv/ENpcBase.csv").toString(),
    {
      from_line: 5,
      on_record: (record, options) => {
        // check each ENpcData for a TripleTriadNpcId

        const npc_card_data = [
          record.data_0,
          record.data_1,
          record.data_2,
          record.data_3,
          record.data_4,
          record.data_5,
          record.data_6,
          record.data_7,
          record.data_8,
          record.data_9,
          record.data_10,
          record.data_11,
          record.data_12,
          record.data_13,
          record.data_14,
          record.data_15,
          record.data_16,
          record.data_17,
          record.data_18,
          record.data_19,
          record.data_20,
          record.data_21,
          record.data_22,
          record.data_23,
          record.data_24,
          record.data_25,
          record.data_26,
          record.data_27,
          record.data_28,
          record.data_29,
          record.data_30,
          record.data_31,
        ].filter((id) => NPCCardMap[id]);

        if (npc_card_data.length > 1) {
          throw new Error(
            `Unexpected multiple triple triad card NPCs [${npc_card_data}] mapped to this NPC [${record.id}]`
          );
        }

        const [TripleTriadNpcId] = npc_card_data;

        if (!TripleTriadNpcId) {
          return null;
        }

        const { id } = record;

        const output_record = { id, TripleTriadNpcId };

        return output_record;
      },

      columns: [
        "id",
        "EventHandler",
        "Important",
        "data_0", // ENpcData[0]
        "data_1", // ENpcData[1]
        "data_2", // ENpcData[2]
        "data_3", // ENpcData[3]
        "data_4", // ENpcData[4]
        "data_5", // ENpcData[5]
        "data_6", // ENpcData[6]
        "data_7", // ENpcData[7]
        "data_8", // ENpcData[8]
        "data_9", // ENpcData[9]
        "data_10", // ENpcData[10]
        "data_11", // ENpcData[11]
        "data_12", // ENpcData[12]
        "data_13", // ENpcData[13]
        "data_14", // ENpcData[14]
        "data_15", // ENpcData[15]
        "data_16", // ENpcData[16]
        "data_17", // ENpcData[17]
        "data_18", // ENpcData[18]
        "data_19", // ENpcData[19]
        "data_20", // ENpcData[20]
        "data_21", // ENpcData[21]
        "data_22", // ENpcData[22]
        "data_23", // ENpcData[23]
        "data_24", // ENpcData[24]
        "data_25", // ENpcData[25]
        "data_26", // ENpcData[26]
        "data_27", // ENpcData[27]
        "data_28", // ENpcData[28]
        "data_29", // ENpcData[29]
        "data_30", // ENpcData[30]
        "data_31", // ENpcData[31]
        "Scale",
        "ModelChara",
        "Race",
        "Gender",
        "BodyType",
        "Height",
        "Tribe",
        "Face",
        "HairStyle",
        "HairHighlight",
        "SkinColor",
        "EyeHeterochromia",
        "HairColor",
        "HairHighlightColor",
        "FacialFeature",
        "FacialFeatureColor",
        "Eyebrows",
        "EyeColor",
        "EyeShape",
        "Nose",
        "Jaw",
        "Mouth",
        "LipColor",
        "BustOrTone1",
        "ExtraFeature1",
        "ExtraFeature2OrBust",
        "FacePaint",
        "FacePaintColor",
        "",
        "NpcEquip",
        "Behavior",
        "Model{MainHand}",
        "Dye{MainHand}",
        "Model{OffHand}",
        "Dye{OffHand}",
        "Model{Head}",
        "Dye{Head}",
        "Visor",
        "Model{Body}",
        "Dye{Body}",
        "Model{Hands}",
        "Dye{Hands}",
        "Model{Legs}",
        "Dye{Legs}",
        "Model{Feet}",
        "Dye{Feet}",
        "Model{Ears}",
        "Dye{Ears}",
        "Model{Neck}",
        "Dye{Neck}",
        "Model{Wrists}",
        "Dye{Wrists}",
        "Model{LeftRing}",
        "Dye{LeftRing}",
        "Model{RightRing}",
        "Dye{RightRing}",
        "Invisibility",
        "Balloon",
        "NotRewriteHeight",
        "DefaultBalloon",
        "",
      ],
    }
  );

  // list.preview(BaseNpcList);
  const BaseNpcMap = list.to_map((c) => c.id, BaseNpcList);

  const BaseNPCNameList = csv.parse(
    fs.readFileSync("./ffxiv-datamining/csv/ENpcResident.csv").toString(),
    {
      from_line: 5,
      on_record: (record, options) => {
        if (!record.name) {
          return null;
        }

        const { id, name } = record;

        const BaseNpc = BaseNpcMap[id];

        if (!BaseNpc) {
          return null;
        }

        const { TripleTriadNpcId } = BaseNpc;

        const output_record = { id, name, TripleTriadNpcId };

        return output_record;
      },

      columns: [
        "id",
        "name",
        "Adjective",
        "Plural",
        "PossessivePronoun",
        "StartsWithVowel",
        "",
        "Pronoun",
        "Article",
        "Title",
        "Map",
        "",
      ],
    }
  );

  // list.preview(BaseNPCNameList);
  const NpcNameMap = list.to_map(
    (npc) => npc.TripleTriadNpcId,
    BaseNPCNameList
  );

  // join back data from BaseNPCNameList to the NPCCardMap
  for (const npc of BaseNPCNameList) {
    const card_npc = NPCCardMap[npc.TripleTriadNpcId];
    card_npc.name = npc.name;
  }

  // console.debug("BaseNPCNameList", BaseNPCNameList.length);
  // console.debug("NPCCardList", NPCCardList.length);

  const final_npc_card_list = [];

  for (const npc of NPCCardList) {
    if (!npc.name) {
      console.error("missing npc", npc);
      continue;
    }

    final_npc_card_list.push(npc);
  }

  // console.debug("final_npc_card_list", final_npc_card_list.length);

  return {
    list: final_npc_card_list,
    map: NPCCardMap,
  };
}

function is_zero(value) {
  return value === "0";
}

function is_not_zero(value) {
  return value !== "0";
}
