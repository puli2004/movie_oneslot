use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;

//One SLotting Joker Movie

			#[status_script(agent = "jack", status = FIGHTER_JACK_STATUS_KIND_FINAL_READY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
			unsafe fn final_ready(fighter: &mut L2CFighterCommon) -> L2CValue {
				let lua_state = fighter.lua_state_agent;
				  if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
				  acmd!(lua_state,{
				let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
				WorkModule::off_flag(module_accessor,*FIGHTER_JACK_STATUS_FINAL_FLAG_CLEAR_SLOW);
				AreaModule::set_whole(module_accessor,false);
				GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
				ItemModule::set_have_item_visibility(module_accessor,false,0);
				WorkModule::on_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
				MotionModule::change_motion(module_accessor,Hash40::new_raw(0x974cac6bcu64),0.0,1.0,false,0.0,false,false);
				WorkModule::set_int(module_accessor,MotionModule::end_frame(module_accessor) as i32,*FIGHTER_JACK_STATUS_FINAL_INT_DASH_FRAME);
				let pointer = fighter.global_table[0x4].get_ptr() as *mut smash::app::Fighter;
				FighterSpecializer_Jack::call_final_module(pointer,*FIGHTER_JACK_FINAL_MODULE_READY_INIT);
				let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
				if sumi_id(module_accessor) == false {
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c00/final_01.h264"));
				}
				else {
				acmd!(lua_state,{
					println!("yes");
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c00/final_00.h264"));
				});
				WorkModule::set_int(module_accessor,*FIGHTER_MOVIE_STATUS_KIND_PREPARE,*FIGHTER_JACK_STATUS_FINAL_INT_MOVIE_STATUS);
				fighter.sub_shift_status_main(L2CValue::Ptr(final_ready_loop as *const () as _));
				L2CValue::I32(0)
			}
		});
	}
		if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
				  acmd!(lua_state,{
				let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
				WorkModule::off_flag(module_accessor,*FIGHTER_JACK_STATUS_FINAL_FLAG_CLEAR_SLOW);
				AreaModule::set_whole(module_accessor,false);
				GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
				ItemModule::set_have_item_visibility(module_accessor,false,0);
				WorkModule::on_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
				MotionModule::change_motion(module_accessor,Hash40::new_raw(0x974cac6bcu64),0.0,1.0,false,0.0,false,false);
				WorkModule::set_int(module_accessor,MotionModule::end_frame(module_accessor) as i32,*FIGHTER_JACK_STATUS_FINAL_INT_DASH_FRAME);
				let pointer = fighter.global_table[0x4].get_ptr() as *mut smash::app::Fighter;
				FighterSpecializer_Jack::call_final_module(pointer,*FIGHTER_JACK_FINAL_MODULE_READY_INIT);
				let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
				if sumi_id(module_accessor) == false {
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c01/final_00.h264"));
				}
				else {
				acmd!(lua_state,{
					println!("yes");
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c01/final_01.h264"));
				});
				WorkModule::set_int(module_accessor,*FIGHTER_MOVIE_STATUS_KIND_PREPARE,*FIGHTER_JACK_STATUS_FINAL_INT_MOVIE_STATUS);
				fighter.sub_shift_status_main(L2CValue::Ptr(final_ready_loop as *const () as _));
				L2CValue::I32(0)
			}
		});
	}
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
					  acmd!(lua_state,{
					let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
					WorkModule::off_flag(module_accessor,*FIGHTER_JACK_STATUS_FINAL_FLAG_CLEAR_SLOW);
					AreaModule::set_whole(module_accessor,false);
					GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
					ItemModule::set_have_item_visibility(module_accessor,false,0);
					WorkModule::on_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
					MotionModule::change_motion(module_accessor,Hash40::new_raw(0x974cac6bcu64),0.0,1.0,false,0.0,false,false);
					WorkModule::set_int(module_accessor,MotionModule::end_frame(module_accessor) as i32,*FIGHTER_JACK_STATUS_FINAL_INT_DASH_FRAME);
					let pointer = fighter.global_table[0x4].get_ptr() as *mut smash::app::Fighter;
					FighterSpecializer_Jack::call_final_module(pointer,*FIGHTER_JACK_FINAL_MODULE_READY_INIT);
					let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
					if sumi_id(module_accessor) == false {
						smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c02/final_00.h264"));
					}
					else {
					acmd!(lua_state,{
						println!("yes");
						smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c02/final_01.h264"));
					});
					WorkModule::set_int(module_accessor,*FIGHTER_MOVIE_STATUS_KIND_PREPARE,*FIGHTER_JACK_STATUS_FINAL_INT_MOVIE_STATUS);
					fighter.sub_shift_status_main(L2CValue::Ptr(final_ready_loop as *const () as _));
					L2CValue::I32(0)
				}
			});
		}
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
				  acmd!(lua_state,{
				let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
				WorkModule::off_flag(module_accessor,*FIGHTER_JACK_STATUS_FINAL_FLAG_CLEAR_SLOW);
				AreaModule::set_whole(module_accessor,false);
				GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
				ItemModule::set_have_item_visibility(module_accessor,false,0);
				WorkModule::on_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
				MotionModule::change_motion(module_accessor,Hash40::new_raw(0x974cac6bcu64),0.0,1.0,false,0.0,false,false);
				WorkModule::set_int(module_accessor,MotionModule::end_frame(module_accessor) as i32,*FIGHTER_JACK_STATUS_FINAL_INT_DASH_FRAME);
				let pointer = fighter.global_table[0x4].get_ptr() as *mut smash::app::Fighter;
				FighterSpecializer_Jack::call_final_module(pointer,*FIGHTER_JACK_FINAL_MODULE_READY_INIT);
				let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
				if sumi_id(module_accessor) == false {
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c03/final_00.h264"));
				}
				else {
				acmd!(lua_state,{
					println!("yes");
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c03/final_01.h264"));
				});
				WorkModule::set_int(module_accessor,*FIGHTER_MOVIE_STATUS_KIND_PREPARE,*FIGHTER_JACK_STATUS_FINAL_INT_MOVIE_STATUS);
				fighter.sub_shift_status_main(L2CValue::Ptr(final_ready_loop as *const () as _));
				L2CValue::I32(0)
			}
		});
	}
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
				  acmd!(lua_state,{
				let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
				WorkModule::off_flag(module_accessor,*FIGHTER_JACK_STATUS_FINAL_FLAG_CLEAR_SLOW);
				AreaModule::set_whole(module_accessor,false);
				GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
				ItemModule::set_have_item_visibility(module_accessor,false,0);
				WorkModule::on_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
				MotionModule::change_motion(module_accessor,Hash40::new_raw(0x974cac6bcu64),0.0,1.0,false,0.0,false,false);
				WorkModule::set_int(module_accessor,MotionModule::end_frame(module_accessor) as i32,*FIGHTER_JACK_STATUS_FINAL_INT_DASH_FRAME);
				let pointer = fighter.global_table[0x4].get_ptr() as *mut smash::app::Fighter;
				FighterSpecializer_Jack::call_final_module(pointer,*FIGHTER_JACK_FINAL_MODULE_READY_INIT);
				let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
				if sumi_id(module_accessor) == false {
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c04/final_00.h264"));
				}
				else {
				acmd!(lua_state,{
					println!("yes");
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c04/final_01.h264"));
				});
				WorkModule::set_int(module_accessor,*FIGHTER_MOVIE_STATUS_KIND_PREPARE,*FIGHTER_JACK_STATUS_FINAL_INT_MOVIE_STATUS);
				fighter.sub_shift_status_main(L2CValue::Ptr(final_ready_loop as *const () as _));
				L2CValue::I32(0)
			}
		});
	}
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
				  acmd!(lua_state,{
				let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
				WorkModule::off_flag(module_accessor,*FIGHTER_JACK_STATUS_FINAL_FLAG_CLEAR_SLOW);
				AreaModule::set_whole(module_accessor,false);
				GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
				ItemModule::set_have_item_visibility(module_accessor,false,0);
				WorkModule::on_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
				MotionModule::change_motion(module_accessor,Hash40::new_raw(0x974cac6bcu64),0.0,1.0,false,0.0,false,false);
				WorkModule::set_int(module_accessor,MotionModule::end_frame(module_accessor) as i32,*FIGHTER_JACK_STATUS_FINAL_INT_DASH_FRAME);
				let pointer = fighter.global_table[0x4].get_ptr() as *mut smash::app::Fighter;
				FighterSpecializer_Jack::call_final_module(pointer,*FIGHTER_JACK_FINAL_MODULE_READY_INIT);
				let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
				if sumi_id(module_accessor) == false {
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c05/final_00.h264"));
				}
				else {
				acmd!(lua_state,{
					println!("yes");
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c05/final_01.h264"));
				});
				WorkModule::set_int(module_accessor,*FIGHTER_MOVIE_STATUS_KIND_PREPARE,*FIGHTER_JACK_STATUS_FINAL_INT_MOVIE_STATUS);
				fighter.sub_shift_status_main(L2CValue::Ptr(final_ready_loop as *const () as _));
				L2CValue::I32(0)
			}
		});
	}
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
				  acmd!(lua_state,{
				let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
				WorkModule::off_flag(module_accessor,*FIGHTER_JACK_STATUS_FINAL_FLAG_CLEAR_SLOW);
				AreaModule::set_whole(module_accessor,false);
				GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
				ItemModule::set_have_item_visibility(module_accessor,false,0);
				WorkModule::on_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
				MotionModule::change_motion(module_accessor,Hash40::new_raw(0x974cac6bcu64),0.0,1.0,false,0.0,false,false);
				WorkModule::set_int(module_accessor,MotionModule::end_frame(module_accessor) as i32,*FIGHTER_JACK_STATUS_FINAL_INT_DASH_FRAME);
				let pointer = fighter.global_table[0x4].get_ptr() as *mut smash::app::Fighter;
				FighterSpecializer_Jack::call_final_module(pointer,*FIGHTER_JACK_FINAL_MODULE_READY_INIT);
				let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
				if sumi_id(module_accessor) == false {
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c06/final_00.h264"));
				}
				else {
				acmd!(lua_state,{
					println!("yes");
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c06/final_01.h264"));
				});
				WorkModule::set_int(module_accessor,*FIGHTER_MOVIE_STATUS_KIND_PREPARE,*FIGHTER_JACK_STATUS_FINAL_INT_MOVIE_STATUS);
				fighter.sub_shift_status_main(L2CValue::Ptr(final_ready_loop as *const () as _));
				L2CValue::I32(0)
			}
		});
	}
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
				  acmd!(lua_state,{
				let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
				WorkModule::off_flag(module_accessor,*FIGHTER_JACK_STATUS_FINAL_FLAG_CLEAR_SLOW);
				AreaModule::set_whole(module_accessor,false);
				GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
				ItemModule::set_have_item_visibility(module_accessor,false,0);
				WorkModule::on_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
				MotionModule::change_motion(module_accessor,Hash40::new_raw(0x974cac6bcu64),0.0,1.0,false,0.0,false,false);
				WorkModule::set_int(module_accessor,MotionModule::end_frame(module_accessor) as i32,*FIGHTER_JACK_STATUS_FINAL_INT_DASH_FRAME);
				let pointer = fighter.global_table[0x4].get_ptr() as *mut smash::app::Fighter;
				FighterSpecializer_Jack::call_final_module(pointer,*FIGHTER_JACK_FINAL_MODULE_READY_INIT);
				let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
				if sumi_id(module_accessor) == false {
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c07/final_00.h264"));
				}
				else {
				acmd!(lua_state,{
					println!("yes");
					smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c07/final_01.h264"));
				});
				WorkModule::set_int(module_accessor,*FIGHTER_MOVIE_STATUS_KIND_PREPARE,*FIGHTER_JACK_STATUS_FINAL_INT_MOVIE_STATUS);
				fighter.sub_shift_status_main(L2CValue::Ptr(final_ready_loop as *const () as _));
				L2CValue::I32(0)
			}
		});
	}
}
