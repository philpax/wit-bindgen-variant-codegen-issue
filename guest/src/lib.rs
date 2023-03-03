pub(crate) mod wit;

struct Guest;
impl wit::guest::Guest for Guest {
    fn init() {
        let identity = wit::types::Mat4 {
            x: wit::types::Vec4 {
                x: 1.,
                y: 0.,
                z: 0.,
                w: 0.,
            },
            y: wit::types::Vec4 {
                x: 0.,
                y: 1.,
                z: 0.,
                w: 0.,
            },
            z: wit::types::Vec4 {
                x: 0.,
                y: 0.,
                z: 1.,
                w: 0.,
            },
            w: wit::types::Vec4 {
                x: 0.,
                y: 0.,
                z: 0.,
                w: 1.,
            },
        };

        let entity = vec![
            (
                0,
                wit::component::ComponentTypeParam::TypeQuat(wit::types::Quat {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                    w: 1.,
                }),
            ),
            (
                1,
                wit::component::ComponentTypeParam::TypeVec3(wit::types::Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 1.,
                }),
            ),
            (
                2,
                wit::component::ComponentTypeParam::TypeVec3(wit::types::Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                }),
            ),
            (3, wit::component::ComponentTypeParam::TypeF32(0.1)),
            (4, wit::component::ComponentTypeParam::TypeMat4(identity)),
            (5, wit::component::ComponentTypeParam::TypeMat4(identity)),
            (6, wit::component::ComponentTypeParam::TypeF32(1.)),
            (7, wit::component::ComponentTypeParam::TypeF32(1.)),
            (8, wit::component::ComponentTypeParam::TypeEmpty(())),
            (9, wit::component::ComponentTypeParam::TypeEmpty(())),
            (10, wit::component::ComponentTypeParam::TypeEmpty(())),
            (
                11,
                wit::component::ComponentTypeParam::TypeVec3(wit::types::Vec3 {
                    x: 5.,
                    y: 5.,
                    z: 4.,
                }),
            ),
            (
                12,
                wit::component::ComponentTypeParam::TypeVec3(wit::types::Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                }),
            ),
            // (
            //     13,
            //     wit::component::ComponentTypeParam::TypeList(
            //         wit::component::ComponentListTypeParam::TypeString(&["Hello", "world!"]),
            //     ),
            // ),
        ];
        println!("Guest: {entity:?}");
        wit::entity::spawn(&entity);
    }

    fn exec(_components: Vec<(u32, wit::guest::ComponentType)>) {}
}

use wit::{__link_section, guest};
wit::export_server!(Guest);
