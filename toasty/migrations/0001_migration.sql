ALTER TABLE "woods" ADD COLUMN "description_heartwood_color_classification" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_green_nail_extraction_parallel_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_narrow_belt_sander_surface_finish" TEXT NOT NULL CHECK ("workabilities_narrow_belt_sander_surface_finish" IN ('not_applicable', 'very_poor', 'poor', 'fair', 'easy', 'good', 'very_good', 'excellent'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "physical_properties_shrinkage_volumetric" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "description_workability_notes" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_narrow_belt_sander_defect" TEXT NOT NULL CHECK ("workabilities_narrow_belt_sander_defect" IN ('not_applicable', 'flawless', 'fair', 'fluffy', 'crushed', 'ripped_off', 'lifted', 'scratched', 'fluffy_and_crushed', 'fluffy_and_ripped_off', 'fluffy_and_scratched', 'fluffy_and_ripped_off_and_lifted', 'fluffy_and_crushed_and_lifted', 'crushed_and_ripped_off', 'crushed_and_fluffy_and_ripped_off', 'lifted_and_ripped_off'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_green_nail_extraction_end_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_green_bending_modulus_of_rupture" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "physical_properties_shrinkage_radial" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "family" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_molding_planer_equipment_settings" TEXT NOT NULL CHECK ("workabilities_molding_planer_equipment_settings" IN ('not_applicable', 'not_suitable', 'suitable'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_heartwood_treatability" TEXT NOT NULL CHECK ("workabilities_heartwood_treatability" IN ('not_applicable', 'green', 'dry', 'very_hard', 'hard', 'moderately_hard', 'moderately_easy', 'easy', 'very_easy'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "physical_properties_shrinkage_tangential" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_dry_janka_hardness_parallel_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "images_log" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_sapwood_treatability" TEXT NOT NULL CHECK ("workabilities_sapwood_treatability" IN ('not_applicable', 'green', 'dry', 'very_hard', 'hard', 'moderately_hard', 'moderately_easy', 'easy', 'very_easy'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_dry_janka_hardness_end_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_narrow_belt_sander_number_of_samples" INTEGER;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_surface_finishing_drill" TEXT NOT NULL CHECK ("workabilities_surface_finishing_drill" IN ('not_applicable', 'very_poor', 'poor', 'fair', 'easy', 'good', 'very_good', 'excellent'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_dry_compression_perpendicular_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "images_bark" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_dry_compression_parallel_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_lathe_surface_finish" TEXT NOT NULL CHECK ("workabilities_lathe_surface_finish" IN ('not_applicable', 'very_poor', 'poor', 'fair', 'easy', 'good', 'very_good', 'excellent'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "images_main" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_green_compression_perpendicular_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_lathe_defect" TEXT NOT NULL CHECK ("workabilities_lathe_defect" IN ('not_applicable', 'flawless', 'fair', 'fluffy', 'crushed', 'ripped_off', 'lifted', 'scratched', 'fluffy_and_crushed', 'fluffy_and_ripped_off', 'fluffy_and_scratched', 'fluffy_and_ripped_off_and_lifted', 'fluffy_and_crushed_and_lifted', 'crushed_and_ripped_off', 'crushed_and_fluffy_and_ripped_off', 'lifted_and_ripped_off'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "source_species_id" INTEGER;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_surface_finishing_nail_crack" TEXT NOT NULL CHECK ("workabilities_surface_finishing_nail_crack" IN ('not_applicable', 'does_not_crack', 'does_not_crack_except_corners', 'cracks'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_lathe_number_of_samples" INTEGER;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_green_janka_hardness_parallel_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "images_tree" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "source_page_url" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_narrow_belt_sander_equipment_settings" TEXT NOT NULL CHECK ("workabilities_narrow_belt_sander_equipment_settings" IN ('not_applicable', 'not_suitable', 'suitable'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "common_names" TEXT NOT NULL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_green_bending_modulus_of_elasticity" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_dry_shear_parallel_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "images_radial_cut" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "collection_location" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_dry_nail_extraction_parallel_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_green_compression_parallel_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "images_wood" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "images_cross_section" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_dry_tension_perpendicular_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_drill_bit_surface_finish" TEXT NOT NULL CHECK ("workabilities_drill_bit_surface_finish" IN ('not_applicable', 'very_poor', 'poor', 'fair', 'easy', 'good', 'very_good', 'excellent'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_green_cleavage_strength" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "images_tangential_cut" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "description_end_uses" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_drill_bit_defect" TEXT NOT NULL CHECK ("workabilities_drill_bit_defect" IN ('not_applicable', 'flawless', 'fair', 'fluffy', 'crushed', 'ripped_off', 'lifted', 'scratched', 'fluffy_and_crushed', 'fluffy_and_ripped_off', 'fluffy_and_scratched', 'fluffy_and_ripped_off_and_lifted', 'fluffy_and_crushed_and_lifted', 'crushed_and_ripped_off', 'crushed_and_fluffy_and_ripped_off', 'lifted_and_ripped_off'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_dry_cleavage_strength" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_green_shear_parallel_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_lathe_equipment_settings" TEXT NOT NULL CHECK ("workabilities_lathe_equipment_settings" IN ('not_applicable', 'not_suitable', 'suitable'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_green_tension_perpendicular_to_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "description_preservation" TEXT;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_green_janka_hardness_end_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_molding_planer_surface_finish" TEXT NOT NULL CHECK ("workabilities_molding_planer_surface_finish" IN ('not_applicable', 'very_poor', 'poor', 'fair', 'easy', 'good', 'very_good', 'excellent'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "physical_properties_density_basic" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "tree" TEXT NOT NULL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_molding_planer_defect" TEXT NOT NULL CHECK ("workabilities_molding_planer_defect" IN ('not_applicable', 'flawless', 'fair', 'fluffy', 'crushed', 'ripped_off', 'lifted', 'scratched', 'fluffy_and_crushed', 'fluffy_and_ripped_off', 'fluffy_and_scratched', 'fluffy_and_ripped_off_and_lifted', 'fluffy_and_crushed_and_lifted', 'crushed_and_ripped_off', 'crushed_and_fluffy_and_ripped_off', 'lifted_and_ripped_off'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_drill_bit_number_of_samples" INTEGER;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_molding_planer_number_of_samples" INTEGER;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "physical_properties_density_apparent" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "general_characteristics" TEXT NOT NULL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_dry_nail_extraction_end_grain" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_surface_finishing_sandpaper" TEXT NOT NULL CHECK ("workabilities_surface_finishing_sandpaper" IN ('not_applicable', 'very_poor', 'poor', 'fair', 'easy', 'good', 'very_good', 'excellent'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "physical_properties_density_dry" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_dry_bending_modulus_of_rupture" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "kiln_drying_schedule" TEXT NOT NULL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_surface_finishing_vise" TEXT NOT NULL CHECK ("workabilities_surface_finishing_vise" IN ('not_applicable', 'very_poor', 'poor', 'fair', 'easy', 'good', 'very_good', 'excellent'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "mechanical_properties_dry_bending_modulus_of_elasticity" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_surface_finishing_planer" TEXT NOT NULL CHECK ("workabilities_surface_finishing_planer" IN ('not_applicable', 'very_poor', 'poor', 'fair', 'easy', 'good', 'very_good', 'excellent'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "workabilities_drill_bit_equipment_settings" TEXT NOT NULL CHECK ("workabilities_drill_bit_equipment_settings" IN ('not_applicable', 'not_suitable', 'suitable'));
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "physical_properties_density_green" REAL;
-- #[toasty::breakpoint]
ALTER TABLE "woods" ADD COLUMN "kiln_drying_days" REAL;
-- #[toasty::breakpoint]
CREATE UNIQUE INDEX "index_woods_by_scientific_name" ON "woods" ("scientific_name");
