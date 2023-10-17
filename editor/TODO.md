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
	
	pub struct Blackboard {
		/// Public expose 여부는 에디터에서 정한다. Public 변수는 어디서든 변경 가능한 
		/// 프로퍼티를 뜻하며, 사용자가 정책에 따라 정의하는 편.
		pub has_line_of_sight: bool,
		
		enemy_actor: Entity,
		patrol_location: Vec3,
	}

	/// 에이전트가 실행해야 하는 액션
	pub enum Task<'a> {
		__Ph(PhantomData<&'a ()>),
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
	
	/// 내부 스테이트 변수 ... 각 노드마다 하나씩 생성된다.
	/// 
	/// 비동기 트리의 경우, 서브트리가 생성
	enum RootState {
		Root,
		/// S 첫 글자 및, 0-9a-zA-Z 문자 중 하나로 지어진 8글자 해시
		SczSSazfg2 {
			
		},
		
	}
	
	
	impl BehaviorTree {
		pub fn blackboard_mut(&mut self) -> &mut Blackboard {
			&mut self.blackboard
		}
		
		pub fn blackboard(&self) -> &Blackboard {
			&self.blackboard
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
	use example_bt::Task;
	
	// 설계에 따라 병렬 실행도 가능할 것
	for agent in agents {
		// 블랙보드는 사전에 업데이트해두거나, 특정 액션의 출력을 연결할 수 있다.
		agent.blackboard_mut().has_line_of_sight = calculate_line_of_sight();
		
		// 행동 트리를 실행한다. 비동기 
		agent.tick(time, |blackboard, action| match action {
				Task::RotateToFaceBBEntry { target } => {
					blackboard.has_line_of_sight = calculate_line_of_sight();
				}
			}
		)
	}
}


```