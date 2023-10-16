# 워크플로

## BT 핵심 개념 적용

ECS, 비동기 평가(웨이커), 시퀀서, 셀랙터, 데코레이터, 블랙보드, 부분 병렬, 웨이커

- ECS와 결합
  - BT는 ECS의 컴포넌트; 즉, 해당 컴포넌트를 포함하는 엔터티가 BT를 물고 있는다.
  - 다시, 각 BT는 블랙보드를 인자로 받는다.
  - 블랙보드는 각 BT마다 정의되는 오브젝트 트레잇; 
- 비동기 평가
  - 매 틱마다 BT를 전체 평가하지 않는다.
  - 


## 편집기 동작

- 파일 단위는 행동 트리
- 행동 트리는 각각 다음을 생성한다:

![BT example](https://docs.unrealengine.com/5.0/Images/making-interactive-experiences/artificial-intelligence/behavior-trees/behavior-tree-overview/behavior-tree-overview-intro.webp)
	
```rust
pub mod example_bt {
	use bevy_bt_core::prelude::*;
	
	/// 빠른 비헤비어 트리 스폰을 위한 번들
	pub type Bundle = BehaviorTree;
	
	/// 적용할 비헤비어 트리
	#[derive(Component)]
	pub struct BehaviorTree {
		action: Option<Action>,
		
		// 비헤비어 트리의 내장 함수 'Wait'을 처리합니다.
		sleep_until: Option<f64>,
		
		// 블랙보드 엔트리가 아래에 생성된다.
		blackboard: Blackboard,
	}
	
	struct Blackboard {
		has_line_of_sight: bool,
		enemy_actor: Entity,
		patrol_location: Vec3,
	}
	
	/// 대부분 데코레이터는 자동 생성될 수 있습니다.

	/// 에이전트가 실행해야 하는 액션
	/// 
	/// 비헤비어 트리는 참고로, 몇 개의 기본 
	pub enum Task<'a> {
		__Ph(PhantomData<&'a ()>),
		ScanLineOfSight {
			has_line_of_sight: &'a bool,
		}
		RotateToFaceBBEntry {
			target: Entity,
		},
		ChasePlayer {
			speed: f64,
		},
		MoveTo {
			target: &'a Vec3,
		},
		FindRandomPatrol {
			patrol_speed: f64,
			patrol_radious: f64,
			out_patrol_location: &'a mut Vec3,
		}
	}
	
	impl BehaviorTree {
		/// 현재 실행중인 액션에 인터럽트를 건다. 트리가 루트부터 다시 평가된다.
		pub fn interrupt(&mut self) {
			self.version += 1;
			self.action = None;
		}

		/// 해당 액션을 실행하고, 액션의 결과를 반환한다; (성공/실패/진행 중)
		/// 
		/// 병렬 태스크의 경우 여러 차례 실행될 수 있다.
		pub fn tick(
			&mut self, 
			time: Res<Time>, 
			runner: impl for<'a> FnMut(Task<'a>) -> Option<bool>
		) 
		{
			// 여기에 비헤비어 트리의 모든 로직이 생성된다.
		}
	}

}

// 유저 예시 시스템:
fn sys_update_bt(
	time: Res<Time>,
	agents: Query<&mut example_bt::BehaviorTree, &mut MyAgentAction>,
) {
	// 설계에 따라 병렬 실행도 가능할 것
	for agent in agents {
		agent.tick(
			time,
			|action| {
				match action {
					RunnerAction::UpdateBlackboard(UpdateBlackboard::HasLineOfSight(has_line_of_sight)) => {
						// 블랙보드 엔트리를 업데이트한다.
						*has_line_of_sight = true;
					},
					
					RunnerAction::Task(task, out_result) => {
						match task {
							Task::RotateToFaceBBEntry { target } => {
								// 블랙보드 엔트리를 읽는다.
								let target = *target;
								
								Some(true)
							}
							...
						}
					}
					
					...
				}
			}
		)
	}
}


```