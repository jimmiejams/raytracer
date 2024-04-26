use std::sync::Arc;

use crate::hittable_list::{HittableList, HittableObject};
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use crate::lambertian_material::LambertianMaterial;
use crate::colour::Colour;
use crate::dialectric_material::DialectricMaterial;
use crate::metal_material::MetalMaterial;

pub fn benchmark_world() -> HittableList {
    let mut world = HittableList::new();
    world.objects = vec![
        HittableObject::Sphere(
            Sphere {
                centre: Vec3 {
                    x: 0.0,
                    y: -1000.0,
                    z: 0.0,
                },
                radius: 1000.0,
                material: Arc::new(LambertianMaterial {
                    albedo: Colour {
                        r: 0.5,
                        g: 0.5,
                        b: 0.5,
                    },
                }),
            },
        ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.268983,
                y: 0.2,
                z: -10.198457,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.43248913,
                    g: 0.67547095,
                    b: 0.18752891,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.183618,
                y: 0.2,
                z: -9.742409,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.085579194,
                    g: 0.0077815764,
                    b: 0.029802904,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.62521,
                y: 0.2,
                z: -8.430011,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.22548671,
                    g: 0.10479941,
                    b: 0.3879952,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.693832,
                y: 0.2,
                z: -7.3198586,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0641399,
                    g: 0.007718018,
                    b: 0.11843904,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.778359,
                y: 0.2,
                z: -6.126798,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.10440301,
                    g: 0.2226699,
                    b: 0.41626072,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.499253,
                y: 0.2,
                z: -5.751897,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0068175164,
                    g: 0.37551484,
                    b: 0.37775734,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.797998,
                y: 0.2,
                z: -4.519468,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.1862439,
                    g: 0.2444265,
                    b: 0.695151,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.410099,
                y: 0.2,
                z: -3.4080691,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.12074359,
                    g: 0.25368115,
                    b: 0.47468236,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.560046,
                y: 0.2,
                z: -2.408001,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.14492005,
                    g: 0.40555713,
                    b: 0.19498923,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.423054,
                y: 0.2,
                z: -1.81269,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.219105,
                y: 0.2,
                z: -0.22088963,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.09938454,
                    g: 0.0016990216,
                    b: 0.09178105,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.115772,
                y: 0.2,
                z: 0.7966709,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.88607097,
                    g: 0.53911144,
                    b: 0.8196102,
                },
                fuzz: 0.18539661,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.120714,
                y: 0.2,
                z: 1.4576991,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.108680844,
                    g: 0.88691545,
                    b: 0.21142948,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.385027,
                y: 0.2,
                z: 2.569086,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.25873905,
                    g: 0.15733352,
                    b: 0.4865686,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.408731,
                y: 0.2,
                z: 3.389977,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.059195627,
                    g: 0.0014294605,
                    b: 0.017472083,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.289419,
                y: 0.2,
                z: 4.430874,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.12847501,
                    g: 0.017326836,
                    b: 0.14488904,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.327451,
                y: 0.2,
                z: 5.5632906,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.72603,
                    g: 0.69545996,
                    b: 0.6622374,
                },
                fuzz: 0.38497508,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.536362,
                y: 0.2,
                z: 6.8897543,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.66761327,
                    g: 0.038367767,
                    b: 0.1955809,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.208847,
                y: 0.2,
                z: 7.5035515,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.21813422,
                    g: 0.40559047,
                    b: 0.65065414,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.413338,
                y: 0.2,
                z: 8.455106,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.078959025,
                    g: 0.7471702,
                    b: 0.120873846,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.39658,
                y: 0.2,
                z: 9.426887,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.11100446,
                    g: 0.28385475,
                    b: 0.013753018,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -10.193439,
                y: 0.2,
                z: 10.378305,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.24861073,
                    g: 0.67226654,
                    b: 0.3096078,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.460486,
                y: 0.2,
                z: -10.51642,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.19332685,
                    g: 0.16218156,
                    b: 0.24293248,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.9927845,
                y: 0.2,
                z: -9.162476,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.13201924,
                    g: 0.08284726,
                    b: 0.041141354,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.465634,
                y: 0.2,
                z: -8.956657,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.1399234,
                    g: 0.13197353,
                    b: 0.43458536,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.468145,
                y: 0.2,
                z: -7.3774395,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.80528605,
                    g: 0.80284667,
                    b: 0.7271782,
                },
                fuzz: 0.2258152,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.671863,
                y: 0.2,
                z: -6.150143,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.12999907,
                    g: 0.44742796,
                    b: 0.54647446,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.181814,
                y: 0.2,
                z: -5.36044,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.49143434,
                    g: 0.20505026,
                    b: 0.031343684,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.809915,
                y: 0.2,
                z: -4.7553062,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.28190807,
                    g: 0.073256075,
                    b: 0.65299773,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.201759,
                y: 0.2,
                z: -3.7401686,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8716669,
                    g: 0.81007266,
                    b: 0.9969187,
                },
                fuzz: 0.16074061,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.32708,
                y: 0.2,
                z: -2.4893942,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.027249519,
                    g: 0.14400534,
                    b: 0.53891444,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.714451,
                y: 0.2,
                z: -1.5536194,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.22453806,
                    g: 0.12747917,
                    b: 0.2108032,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.472582,
                y: 0.2,
                z: -0.8758446,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.3250356,
                    g: 0.046077695,
                    b: 0.44838142,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.9386015,
                y: 0.2,
                z: 0.78309655,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.934636,
                    g: 0.9221988,
                    b: 0.5005517,
                },
                fuzz: 0.45176393,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.85728,
                y: 0.2,
                z: 1.6273017,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.14400135,
                    g: 0.0042089378,
                    b: 0.72384775,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.791956,
                y: 0.2,
                z: 2.8341568,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.3262064,
                    g: 0.25871527,
                    b: 0.2990519,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.584264,
                y: 0.2,
                z: 3.1684623,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.040770657,
                    g: 0.44880813,
                    b: 0.5851043,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.775666,
                y: 0.2,
                z: 4.7510557,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.20267078,
                    g: 0.18622848,
                    b: 0.090969466,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.358059,
                y: 0.2,
                z: 5.5846663,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.10004111,
                    g: 0.1731566,
                    b: 0.3988244,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.834647,
                y: 0.2,
                z: 6.1273556,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.20378453,
                    g: 0.13772163,
                    b: 0.34821314,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.446418,
                y: 0.2,
                z: 7.2062855,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.23783721,
                    g: 0.49806878,
                    b: 0.425122,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.865718,
                y: 0.2,
                z: 8.553986,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0018263607,
                    g: 0.2017421,
                    b: 0.2796133,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.480175,
                y: 0.2,
                z: 9.093723,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.017492853,
                    g: 0.23468798,
                    b: 0.0019844156,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -9.70571,
                y: 0.2,
                z: 10.289757,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.15581885,
                    g: 0.028104763,
                    b: 0.538636,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.559968,
                y: 0.2,
                z: -10.281828,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.07584121,
                    g: 0.03743383,
                    b: 0.08677658,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.398685,
                y: 0.2,
                z: -9.656864,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.09989995,
                    g: 0.03447674,
                    b: 0.07996089,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.914469,
                y: 0.2,
                z: -8.375007,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.29939508,
                    g: 0.017057808,
                    b: 0.29513577,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.648868,
                y: 0.2,
                z: -7.2173347,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.7261852,
                    g: 0.16766137,
                    b: 0.069527596,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.287493,
                y: 0.2,
                z: -6.8665886,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.23130286,
                    g: 0.020990502,
                    b: 0.27297047,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.93136,
                y: 0.2,
                z: -5.6368437,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.2748042,
                    g: 0.54784995,
                    b: 0.044234626,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.194192,
                y: 0.2,
                z: -4.545103,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.58260083,
                    g: 0.5351514,
                    b: 0.32327968,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.255435,
                y: 0.2,
                z: -3.6971767,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.27502468,
                    g: 0.24937874,
                    b: 0.28379428,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.684476,
                y: 0.2,
                z: -2.8463137,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.61727864,
                    g: 0.050180107,
                    b: 0.17118497,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.150833,
                y: 0.2,
                z: -1.7399784,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.928897,
                    g: 0.75228524,
                    b: 0.5507693,
                },
                fuzz: 0.25825828,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.59538,
                y: 0.2,
                z: -0.6244518,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.75942993,
                    g: 0.7456721,
                    b: 0.7534384,
                },
                fuzz: 0.13546687,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.242344,
                y: 0.2,
                z: 0.19704156,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.028701391,
                    g: 0.12948395,
                    b: 0.16613911,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.284288,
                y: 0.2,
                z: 1.5547072,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0122610135,
                    g: 0.20876639,
                    b: 0.28456643,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.198761,
                y: 0.2,
                z: 2.2285793,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.29059297,
                    g: 0.5338908,
                    b: 0.02988289,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.665582,
                y: 0.2,
                z: 3.3192992,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.45109904,
                    g: 0.73698485,
                    b: 0.3490727,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.893743,
                y: 0.2,
                z: 4.705746,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8162377,
                    g: 0.73023796,
                    b: 0.5729175,
                },
                fuzz: 0.3995313,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.30325,
                y: 0.2,
                z: 5.3287573,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.9423467,
                    g: 0.92388296,
                    b: 0.84564614,
                },
                fuzz: 0.16849211,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.31402,
                y: 0.2,
                z: 6.0843368,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.41383973,
                    g: 0.4185802,
                    b: 0.418854,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.485818,
                y: 0.2,
                z: 7.4286404,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.34548295,
                    g: 0.063731164,
                    b: 0.4784698,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.400771,
                y: 0.2,
                z: 8.317763,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.48545554,
                    g: 0.10942736,
                    b: 0.2389947,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.454905,
                y: 0.2,
                z: 9.329116,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.38756824,
                    g: 0.10367774,
                    b: 0.03780159,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -8.984642,
                y: 0.2,
                z: 10.010679,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.2817277,
                    g: 0.8312546,
                    b: 0.11702779,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.531585,
                y: 0.2,
                z: -10.639968,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.2712679,
                    g: 0.0105444165,
                    b: 0.43043566,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.290706,
                y: 0.2,
                z: -9.445312,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.15703227,
                    g: 0.17738393,
                    b: 0.7491447,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.9395027,
                y: 0.2,
                z: -8.38512,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.10856752,
                    g: 0.88317335,
                    b: 0.41752306,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.541812,
                y: 0.2,
                z: -7.777327,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.066267416,
                    g: 0.33887035,
                    b: 0.3755265,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.2197876,
                y: 0.2,
                z: -6.385383,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.20712504,
                    g: 0.1800399,
                    b: 0.28749195,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.967963,
                y: 0.2,
                z: -5.905129,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.25342768,
                    g: 0.2340975,
                    b: 0.058348726,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.8658714,
                y: 0.2,
                z: -4.6640234,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.027975824,
                    g: 0.3558548,
                    b: 0.30815095,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.280008,
                y: 0.2,
                z: -3.7139013,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.31458664,
                    g: 0.34752062,
                    b: 0.5504238,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.759696,
                y: 0.2,
                z: -2.2593317,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.20080614,
                    g: 0.45508236,
                    b: 0.6328062,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.5767984,
                y: 0.2,
                z: -1.4390218,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.19976105,
                    g: 0.48792514,
                    b: 0.740844,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.7602015,
                y: 0.2,
                z: -0.52618855,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.14673448,
                    g: 0.003769276,
                    b: 0.54696435,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.632809,
                y: 0.2,
                z: 0.66093934,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.03768599,
                    g: 0.05945797,
                    b: 0.039968364,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.2976336,
                y: 0.2,
                z: 1.8974384,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.21738419,
                    g: 0.17620987,
                    b: 0.024539307,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.1732745,
                y: 0.2,
                z: 2.7389932,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.48316216,
                    g: 0.2739984,
                    b: 0.02979387,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.8932796,
                y: 0.2,
                z: 3.7578073,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7631899,
                    g: 0.79020905,
                    b: 0.98938996,
                },
                fuzz: 0.44842416,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.5438814,
                y: 0.2,
                z: 4.2379074,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7047704,
                    g: 0.6409352,
                    b: 0.70190203,
                },
                fuzz: 0.20105672,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.212964,
                y: 0.2,
                z: 5.8382215,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.2383622,
                    g: 0.0963536,
                    b: 0.12163051,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.601151,
                y: 0.2,
                z: 6.2085643,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.7002005,
                    g: 0.06424571,
                    b: 0.15458308,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.21821,
                y: 0.2,
                z: 7.0635133,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.051539704,
                    g: 0.14691918,
                    b: 0.48784372,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.898367,
                y: 0.2,
                z: 8.038224,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.56799567,
                    g: 0.668614,
                    b: 0.88442,
                },
                fuzz: 0.2850356,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.968252,
                y: 0.2,
                z: 9.744136,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0077029904,
                    g: 0.4285337,
                    b: 0.08756962,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -7.594281,
                y: 0.2,
                z: 10.118104,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.8008513,
                    g: 0.7448175,
                    b: 0.06649566,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.299749,
                y: 0.2,
                z: -10.909487,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.436197,
                    g: 0.7622513,
                    b: 0.0879357,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.9234085,
                y: 0.2,
                z: -9.894465,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.55510044,
                    g: 0.9468885,
                    b: 0.8329489,
                },
                fuzz: 0.16674608,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.2782917,
                y: 0.2,
                z: -8.579269,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.5763244,
                    g: 0.21555309,
                    b: 0.3579589,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.7084703,
                y: 0.2,
                z: -7.6066384,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.11100722,
                    g: 0.032075707,
                    b: 0.10164885,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.2779045,
                y: 0.2,
                z: -6.8014255,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.067859836,
                    g: 0.078530096,
                    b: 0.39943343,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.1838217,
                y: 0.2,
                z: -5.8159976,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.425352,
                y: 0.2,
                z: -4.6572523,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.3359088,
                    g: 0.08544785,
                    b: 0.23814082,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.500899,
                y: 0.2,
                z: -3.1828408,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.41255707,
                    g: 0.34549433,
                    b: 0.68022764,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.588923,
                y: 0.2,
                z: -2.8253272,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.4741142,
                    g: 0.2300136,
                    b: 0.2512364,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.3765383,
                y: 0.2,
                z: -1.8342309,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.038360555,
                    g: 0.1889515,
                    b: 0.27741677,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.303109,
                y: 0.2,
                z: -0.8092288,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.12188057,
                    g: 0.12633279,
                    b: 0.017872,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.7011914,
                y: 0.2,
                z: 0.75675684,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8471503,
                    g: 0.7880806,
                    b: 0.8341488,
                },
                fuzz: 0.3930437,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.8306065,
                y: 0.2,
                z: 1.4330018,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.025775312,
                    g: 0.06832865,
                    b: 0.052860636,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.313658,
                y: 0.2,
                z: 2.4742918,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.09900121,
                    g: 0.1527459,
                    b: 0.28086,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.1071696,
                y: 0.2,
                z: 3.6688666,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0075378073,
                    g: 0.14858384,
                    b: 0.06423004,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.707608,
                y: 0.2,
                z: 4.710855,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7067145,
                    g: 0.66716874,
                    b: 0.5988417,
                },
                fuzz: 0.0038905442,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.524292,
                y: 0.2,
                z: 5.298754,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.1204524,
                    g: 0.11127953,
                    b: 0.27304497,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.7877893,
                y: 0.2,
                z: 6.023025,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.97462213,
                    g: 0.934656,
                    b: 0.65566146,
                },
                fuzz: 0.4290027,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.892524,
                y: 0.2,
                z: 7.066691,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.99630815,
                    g: 0.85659015,
                    b: 0.70631707,
                },
                fuzz: 0.13842532,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.4194775,
                y: 0.2,
                z: 8.371736,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.53403986,
                    g: 0.5483743,
                    b: 0.5322621,
                },
                fuzz: 0.24597448,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.403487,
                y: 0.2,
                z: 9.059049,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -6.285859,
                y: 0.2,
                z: 10.53601,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.67001927,
                    g: 0.98063767,
                    b: 0.9748647,
                },
                fuzz: 0.025532901,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.9025326,
                y: 0.2,
                z: -10.973362,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.545111,
                    g: 0.09250291,
                    b: 0.14441048,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.1719847,
                y: 0.2,
                z: -9.880898,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.050069492,
                    g: 0.6023564,
                    b: 0.5182607,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.107481,
                y: 0.2,
                z: -8.9442835,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.06853679,
                    g: 0.026955154,
                    b: 0.6219247,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.201951,
                y: 0.2,
                z: -7.81581,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0023722425,
                    g: 0.20673756,
                    b: 0.015650172,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.109407,
                y: 0.2,
                z: -6.681617,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.5183465,
                    g: 0.8220963,
                    b: 0.98731923,
                },
                fuzz: 0.43426824,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.855942,
                y: 0.2,
                z: -5.854641,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.56354856,
                    g: 0.051581226,
                    b: 0.047815207,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.517356,
                y: 0.2,
                z: -4.1237154,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.69113564,
                    g: 0.93934643,
                    b: 0.7681396,
                },
                fuzz: 0.15876573,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.4442463,
                y: 0.2,
                z: -3.9568324,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.029637404,
                    g: 0.39827415,
                    b: 0.064543396,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.582671,
                y: 0.2,
                z: -2.6848598,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.28787398,
                    g: 0.18445015,
                    b: 0.40781003,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.139285,
                y: 0.2,
                z: -1.2000797,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.22903475,
                    g: 0.48502883,
                    b: 0.03320261,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.842364,
                y: 0.2,
                z: -0.6061673,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.1682389,
                    g: 0.108961195,
                    b: 0.23721425,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.7705717,
                y: 0.2,
                z: 0.49487555,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.5482805,
                    g: 0.61084664,
                    b: 0.7119771,
                },
                fuzz: 0.43811128,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.510052,
                y: 0.2,
                z: 1.8016679,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.08119645,
                    g: 0.6236431,
                    b: 0.12187926,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.2822766,
                y: 0.2,
                z: 2.218975,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.1590203,
                    g: 0.22813785,
                    b: 0.19642353,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.6746087,
                y: 0.2,
                z: 3.201287,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.08066193,
                    g: 0.0029398983,
                    b: 0.27271402,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.553285,
                y: 0.2,
                z: 4.4247165,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.32148734,
                    g: 0.15866438,
                    b: 0.06540656,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.8776517,
                y: 0.2,
                z: 5.634709,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.9018306,
                    g: 0.5692307,
                    b: 0.65455997,
                },
                fuzz: 0.3391918,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.363613,
                y: 0.2,
                z: 6.1872506,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.30201134,
                    g: 0.037399244,
                    b: 0.047287803,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.8339887,
                y: 0.2,
                z: 7.060711,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.54358137,
                    g: 0.8354113,
                    b: 0.9224888,
                },
                fuzz: 0.24794441,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.8690667,
                y: 0.2,
                z: 8.48633,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.4795083,
                    g: 0.51727,
                    b: 0.21367477,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.571714,
                y: 0.2,
                z: 9.634606,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.09035442,
                    g: 0.32860342,
                    b: 0.24273518,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -5.4001184,
                y: 0.2,
                z: 10.85586,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.53203607,
                    g: 0.11601632,
                    b: 0.09087359,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.482309,
                y: 0.2,
                z: -10.604823,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.75385225,
                    g: 0.52236694,
                    b: 0.79306006,
                },
                fuzz: 0.37642822,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.5907626,
                y: 0.2,
                z: -9.646663,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.26758742,
                    g: 0.0068466356,
                    b: 0.3195971,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.1721315,
                y: 0.2,
                z: -8.201268,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.29316252,
                    g: 0.6117583,
                    b: 0.025994815,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.842153,
                y: 0.2,
                z: -7.5209208,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.08122545,
                    g: 0.4399575,
                    b: 0.21540274,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.743578,
                y: 0.2,
                z: -6.876521,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.08658106,
                    g: 0.019330733,
                    b: 0.030262683,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.3447676,
                y: 0.2,
                z: -5.6028504,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.27802593,
                    g: 0.05400916,
                    b: 0.22350359,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.3550215,
                y: 0.2,
                z: -4.1464815,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.048877392,
                    g: 0.18349789,
                    b: 0.30813795,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.176248,
                y: 0.2,
                z: -3.367155,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.51829857,
                    g: 0.68881077,
                    b: 0.64461887,
                },
                fuzz: 0.25150102,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.3402805,
                y: 0.2,
                z: -2.9472952,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.04407241,
                    g: 0.020649359,
                    b: 0.14419483,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.7935734,
                y: 0.2,
                z: -1.6123455,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.544756,
                    g: 0.049489956,
                    b: 0.02048709,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.506424,
                y: 0.2,
                z: -0.35888416,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 8.000108e-5,
                    g: 0.47226742,
                    b: 0.7352277,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.273202,
                y: 0.2,
                z: 0.111441165,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.4632887,
                    g: 0.060472135,
                    b: 0.042810876,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.6008835,
                y: 0.2,
                z: 1.014357,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.23547599,
                    g: 0.33825144,
                    b: 0.1382937,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.6216655,
                y: 0.2,
                z: 2.7405388,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8553679,
                    g: 0.9883978,
                    b: 0.52015936,
                },
                fuzz: 0.23784235,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.13833,
                y: 0.2,
                z: 3.5376308,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.63866,
                    g: 0.0032386975,
                    b: 0.03609513,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.4362903,
                y: 0.2,
                z: 4.636509,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.1208926,
                    g: 0.582307,
                    b: 0.10341917,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.5532913,
                y: 0.2,
                z: 5.398939,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.014323415,
                    g: 0.016464433,
                    b: 0.063429005,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.5958643,
                y: 0.2,
                z: 6.2355614,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.001922471,
                    g: 0.13098952,
                    b: 0.016701574,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.188145,
                y: 0.2,
                z: 7.2044697,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.49719265,
                    g: 0.16666062,
                    b: 0.07134361,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.715502,
                y: 0.2,
                z: 8.053498,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.02844666,
                    g: 0.16294774,
                    b: 0.64879936,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.3218007,
                y: 0.2,
                z: 9.799508,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.13493824,
                    g: 0.22554062,
                    b: 0.08849018,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.202928,
                y: 0.2,
                z: 10.5215225,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.013158441,
                    g: 0.05863372,
                    b: 0.32234326,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.4233882,
                y: 0.2,
                z: -10.298256,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.6225558,
                    g: 0.2373581,
                    b: 0.5864649,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.3354423,
                y: 0.2,
                z: -9.542667,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.6252356,
                    g: 0.020678747,
                    b: 0.41671875,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.9468713,
                y: 0.2,
                z: -8.324806,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0414812,
                    g: 0.0011662135,
                    b: 0.36353877,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.686977,
                y: 0.2,
                z: -7.682144,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.62269473,
                    g: 0.37171623,
                    b: 0.0661295,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.3673663,
                y: 0.2,
                z: -6.1116943,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.9179028,
                    g: 0.9312134,
                    b: 0.5889683,
                },
                fuzz: 0.29887646,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.6149244,
                y: 0.2,
                z: -5.9079795,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.6959744,
                    g: 0.54245794,
                    b: 0.94733465,
                },
                fuzz: 0.20902023,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.4796877,
                y: 0.2,
                z: -4.813244,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.90066445,
                    g: 0.008405418,
                    b: 0.58389026,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.399185,
                y: 0.2,
                z: -3.6973002,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.9904683,
                    g: 0.61652565,
                    b: 0.6407403,
                },
                fuzz: 0.38403594,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.741443,
                y: 0.2,
                z: -2.7255197,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.23668277,
                    g: 0.6435583,
                    b: 0.032057673,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.4627829,
                y: 0.2,
                z: -1.1547904,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.014168513,
                    g: 0.12925093,
                    b: 0.6205335,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.6980343,
                y: 0.2,
                z: -0.84111255,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.18127382,
                    g: 0.17813604,
                    b: 0.7437903,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.6135936,
                y: 0.2,
                z: 0.72087264,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.4030153,
                    g: 0.19322969,
                    b: 0.08854753,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.9812114,
                y: 0.2,
                z: 1.6842395,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0023689275,
                    g: 0.13350523,
                    b: 0.36242524,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.552979,
                y: 0.2,
                z: 2.4183416,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.5315293,
                    g: 0.5126605,
                    b: 0.87967116,
                },
                fuzz: 0.3880283,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.7661076,
                y: 0.2,
                z: 3.3801048,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.23201,
                y: 0.2,
                z: 4.5165343,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.93598384,
                    g: 0.69977087,
                    b: 0.62896323,
                },
                fuzz: 0.23985076,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.1829545,
                y: 0.2,
                z: 5.1934466,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.009128331,
                    g: 0.13275072,
                    b: 0.41682577,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.8440866,
                y: 0.2,
                z: 6.639241,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.04418725,
                    g: 0.40852797,
                    b: 0.34004006,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.6912067,
                y: 0.2,
                z: 7.609731,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.66123664,
                    g: 0.36646488,
                    b: 0.036949135,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.55291,
                y: 0.2,
                z: 8.087926,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.19671707,
                    g: 0.5393152,
                    b: 0.46193802,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.722459,
                y: 0.2,
                z: 9.808153,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.5912435,
                    g: 0.87576294,
                    b: 0.70258033,
                },
                fuzz: 0.16468462,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -3.2759948,
                y: 0.2,
                z: 10.435613,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.34323928,
                    g: 0.13204315,
                    b: 0.037415784,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.4695635,
                y: 0.2,
                z: -10.988398,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.08137913,
                    g: 0.25039944,
                    b: 0.22838832,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.4274476,
                y: 0.2,
                z: -9.743131,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.90725225,
                    g: 0.1520824,
                    b: 0.5359499,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.439068,
                y: 0.2,
                z: -8.2336235,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.28995433,
                    g: 0.35751304,
                    b: 0.03226586,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.2827024,
                y: 0.2,
                z: -7.385484,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.33383238,
                    g: 0.0276254,
                    b: 0.013282752,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.8102694,
                y: 0.2,
                z: -6.5103655,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.007635569,
                    g: 0.36738676,
                    b: 0.200167,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.9371827,
                y: 0.2,
                z: -5.3083687,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.018214501,
                    g: 0.1336885,
                    b: 4.2520904e-5,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.665884,
                y: 0.2,
                z: -4.7238703,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.22782415,
                    g: 0.2007863,
                    b: 0.5341184,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.452022,
                y: 0.2,
                z: -3.914231,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.000542315,
                    g: 0.032807514,
                    b: 0.5282989,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.7139368,
                y: 0.2,
                z: -2.2472458,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.022915637,
                    g: 0.08327977,
                    b: 0.038025007,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.706103,
                y: 0.2,
                z: -1.7009597,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.12378645,
                    g: 0.43901443,
                    b: 0.02119978,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.3235698,
                y: 0.2,
                z: -0.82411146,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.53373563,
                    g: 0.009038217,
                    b: 0.6187974,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.5593867,
                y: 0.2,
                z: 0.88894093,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.56038344,
                    g: 0.19442461,
                    b: 0.16229542,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.8501644,
                y: 0.2,
                z: 1.6265645,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.39403355,
                    g: 0.56255364,
                    b: 0.093066975,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.1818404,
                y: 0.2,
                z: 2.7769933,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.16397376,
                    g: 0.23416367,
                    b: 0.4694684,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.7132037,
                y: 0.2,
                z: 3.733873,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.6119579,
                    g: 0.05429244,
                    b: 0.107926026,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.488223,
                y: 0.2,
                z: 4.669017,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.56393623,
                    g: 0.2367959,
                    b: 0.071626976,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.6598697,
                y: 0.2,
                z: 5.6068645,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.6391624,
                    g: 0.9660539,
                    b: 0.5633062,
                },
                fuzz: 0.38564813,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.4104965,
                y: 0.2,
                z: 6.7132,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.61281276,
                    g: 0.029219085,
                    b: 0.01945754,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.260617,
                y: 0.2,
                z: 7.155874,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.042987004,
                    g: 0.689174,
                    b: 0.3672229,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.5527167,
                y: 0.2,
                z: 8.891811,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.07723015,
                    g: 0.010705473,
                    b: 0.30808938,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.3059916,
                y: 0.2,
                z: 9.145781,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.07028635,
                    g: 0.03707287,
                    b: 0.36120394,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -2.5562341,
                y: 0.2,
                z: 10.032051,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.99725527,
                    g: 0.8462845,
                    b: 0.8930701,
                },
                fuzz: 0.2078684,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.7528609,
                y: 0.2,
                z: -10.837849,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8151453,
                    g: 0.8215126,
                    b: 0.9704876,
                },
                fuzz: 0.49472085,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.3446984,
                y: 0.2,
                z: -9.899168,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.30096403,
                    g: 0.5504928,
                    b: 0.06066034,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.9717737,
                y: 0.2,
                z: -8.307616,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.04345527,
                    g: 0.24299335,
                    b: 0.0134548135,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.6317691,
                y: 0.2,
                z: -7.407004,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.029199429,
                    g: 0.060829207,
                    b: 0.23045672,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.666415,
                y: 0.2,
                z: -6.8084645,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.23716463,
                    g: 0.69568694,
                    b: 0.33213928,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.7639774,
                y: 0.2,
                z: -5.8250275,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.084428005,
                    g: 0.019477554,
                    b: 0.8563897,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.1151597,
                y: 0.2,
                z: -4.3747845,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 6.308885e-5,
                    g: 0.24046965,
                    b: 0.3560443,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.8576049,
                y: 0.2,
                z: -3.1787508,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7475966,
                    g: 0.9039812,
                    b: 0.7804971,
                },
                fuzz: 0.34077397,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.1260464,
                y: 0.2,
                z: -2.435918,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.26306975,
                    g: 0.18044046,
                    b: 0.26984954,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.7590966,
                y: 0.2,
                z: -1.9172142,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0096266745,
                    g: 0.46391627,
                    b: 0.06683281,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.6911845,
                y: 0.2,
                z: -0.93857485,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.14118345,
                    g: 0.67505074,
                    b: 0.82209605,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.3159628,
                y: 0.2,
                z: 0.59083563,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.71790636,
                    g: 0.80604124,
                    b: 0.63773465,
                },
                fuzz: 0.29404798,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.2660464,
                y: 0.2,
                z: 1.6782211,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.12814854,
                    g: 0.42862347,
                    b: 0.49631906,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.5274193,
                y: 0.2,
                z: 2.74431,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8632271,
                    g: 0.82171667,
                    b: 0.7757249,
                },
                fuzz: 0.48264158,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.4321287,
                y: 0.2,
                z: 3.7931805,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.008873312,
                    g: 0.17452776,
                    b: 0.50863934,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.2184806,
                y: 0.2,
                z: 4.4183946,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8329829,
                    g: 0.6594652,
                    b: 0.7035645,
                },
                fuzz: 0.052206814,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.8897281,
                y: 0.2,
                z: 5.423809,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.9124478,
                    g: 0.8698954,
                    b: 0.9084301,
                },
                fuzz: 0.47016308,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.3342749,
                y: 0.2,
                z: 6.853639,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.5883765,
                    g: 0.45860708,
                    b: 0.1761093,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.5389531,
                y: 0.2,
                z: 7.2916074,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.26820254,
                    g: 0.22493267,
                    b: 0.13667248,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.319528,
                y: 0.2,
                z: 8.737693,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.133199,
                y: 0.2,
                z: 9.682812,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7766498,
                    g: 0.6747776,
                    b: 0.5815891,
                },
                fuzz: 0.19787511,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -1.113879,
                y: 0.2,
                z: 10.455923,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.29781553,
                    g: 0.004273262,
                    b: 0.07012293,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.7094535,
                y: 0.2,
                z: -10.761822,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.3718621,
                    g: 0.33701596,
                    b: 0.08837703,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.6806061,
                y: 0.2,
                z: -9.932569,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 4.1977215e-5,
                    g: 0.556316,
                    b: 0.02611319,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.6571915,
                y: 0.2,
                z: -8.198637,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.21568947,
                    g: 0.006523856,
                    b: 0.004088261,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.46315867,
                y: 0.2,
                z: -7.395608,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.013435866,
                    g: 0.035059337,
                    b: 0.2590588,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.66073847,
                y: 0.2,
                z: -6.964641,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.24798448,
                    g: 0.65149367,
                    b: 0.19901524,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.49970227,
                y: 0.2,
                z: -5.6256504,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.2030747,
                    g: 0.44150218,
                    b: 0.35417134,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.8840127,
                y: 0.2,
                z: -4.339875,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.16809964,
                    g: 0.19665584,
                    b: 0.12340542,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.24199903,
                y: 0.2,
                z: -3.8378563,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.25424916,
                    g: 0.26659325,
                    b: 0.004992755,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.5836737,
                y: 0.2,
                z: -2.863418,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.30124998,
                    g: 0.13506609,
                    b: 0.36719096,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.5050672,
                y: 0.2,
                z: -1.8273785,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.044939335,
                    g: 0.86301064,
                    b: 0.1993649,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.12810874,
                y: 0.2,
                z: -0.39216948,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.18442596,
                    g: 0.020848978,
                    b: 0.53020304,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.52091795,
                y: 0.2,
                z: 0.27865148,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.33876267,
                    g: 0.22662818,
                    b: 0.22433461,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.82027286,
                y: 0.2,
                z: 1.5596809,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.42633575,
                    g: 0.49895418,
                    b: 0.104970194,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.42721146,
                y: 0.2,
                z: 2.3630326,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.68996274,
                y: 0.2,
                z: 3.4079015,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.47484568,
                    g: 0.046552792,
                    b: 0.10150867,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.8930541,
                y: 0.2,
                z: 4.2362795,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.36117074,
                    g: 0.07246801,
                    b: 0.72115463,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.99992675,
                y: 0.2,
                z: 5.6246204,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.08932631,
                    g: 0.0726983,
                    b: 0.21665417,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.79410744,
                y: 0.2,
                z: 6.3027425,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.024931947,
                    g: 0.19460863,
                    b: 0.11067939,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.7426934,
                y: 0.2,
                z: 7.7342143,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.5255349,
                y: 0.2,
                z: 8.301265,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.62000203,
                    g: 0.5662882,
                    b: 0.18135601,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.9328912,
                y: 0.2,
                z: 9.838187,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -0.9134927,
                y: 0.2,
                z: 10.292325,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.1430727,
                    g: 0.6780822,
                    b: 0.4702359,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.008893722,
                y: 0.2,
                z: -10.541203,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.8075411,
                    g: 0.8511986,
                    b: 0.49236017,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.64876586,
                y: 0.2,
                z: -9.708817,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.04195474,
                    g: 0.056486774,
                    b: 0.120712005,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.12369666,
                y: 0.2,
                z: -8.810875,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.21237603,
                    g: 0.16762285,
                    b: 0.43976673,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.56047773,
                y: 0.2,
                z: -7.8627396,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.6024193,
                    g: 0.62898374,
                    b: 0.7323792,
                },
                fuzz: 0.3399494,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.36373034,
                y: 0.2,
                z: -6.383019,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.22608517,
                    g: 0.019968621,
                    b: 0.093246244,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.5955138,
                y: 0.2,
                z: -5.1995015,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.05317553,
                    g: 0.48881263,
                    b: 0.101901814,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.85659456,
                y: 0.2,
                z: -4.3103433,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.09636005,
                    g: 0.017233135,
                    b: 0.19182406,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.86938983,
                y: 0.2,
                z: -3.717232,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.59409684,
                    g: 0.25646153,
                    b: 0.0127652185,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.1059266,
                y: 0.2,
                z: -2.512533,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.13689104,
                    g: 0.6279078,
                    b: 0.0073760436,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.7357488,
                y: 0.2,
                z: -1.4548275,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.02948868,
                    g: 0.0308365,
                    b: 0.07197607,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.02761243,
                y: 0.2,
                z: -0.6750976,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.45321906,
                    g: 0.34495112,
                    b: 0.32783416,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.63570714,
                y: 0.2,
                z: 0.8629275,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.21581656,
                    g: 0.04513412,
                    b: 0.2928798,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.16336401,
                y: 0.2,
                z: 1.1878215,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.6853382,
                    g: 0.21224448,
                    b: 0.07974921,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.088154554,
                y: 0.2,
                z: 2.8525782,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.16509175,
                    g: 0.28412038,
                    b: 0.010260922,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.8164423,
                y: 0.2,
                z: 3.5861094,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.21796557,
                    g: 0.3100394,
                    b: 0.35240582,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.28992024,
                y: 0.2,
                z: 4.12052,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.01671481,
                    g: 0.39522144,
                    b: 0.28760383,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.351205,
                y: 0.2,
                z: 5.7399206,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.28434768,
                    g: 0.105293065,
                    b: 0.18074518,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.53015244,
                y: 0.2,
                z: 6.36393,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.12169286,
                    g: 0.02048148,
                    b: 0.59984374,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.44124,
                y: 0.2,
                z: 7.281561,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.26423848,
                    g: 0.05219882,
                    b: 0.046542797,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.2409159,
                y: 0.2,
                z: 8.474809,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.30867004,
                    g: 0.7844553,
                    b: 0.5478903,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.5085984,
                y: 0.2,
                z: 9.090961,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.56057984,
                    g: 0.5867589,
                    b: 0.618633,
                },
                fuzz: 0.09951234,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.40815362,
                y: 0.2,
                z: 10.68869,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8542743,
                    g: 0.60035217,
                    b: 0.8692994,
                },
                fuzz: 0.38941157,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.0480243,
                y: 0.2,
                z: -10.467782,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7854455,
                    g: 0.7496388,
                    b: 0.9911698,
                },
                fuzz: 0.11083156,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.0555017,
                y: 0.2,
                z: -9.579265,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.03382257,
                    g: 0.13550013,
                    b: 0.013672968,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.120236,
                y: 0.2,
                z: -8.862962,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.23373553,
                    g: 0.5388709,
                    b: 0.080677874,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.0831627,
                y: 0.2,
                z: -7.5038066,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.40833324,
                    g: 0.13626568,
                    b: 0.061376736,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.8572729,
                y: 0.2,
                z: -6.9709015,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0310498,
                    g: 0.22108737,
                    b: 0.08839975,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.8848972,
                y: 0.2,
                z: -5.638324,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.6095254,
                    g: 0.5239793,
                    b: 0.37071684,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.1638083,
                y: 0.2,
                z: -4.3220816,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.7438132,
                    g: 0.45173436,
                    b: 0.06468543,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.3875531,
                y: 0.2,
                z: -3.6909363,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.58563876,
                    g: 0.53833294,
                    b: 0.5208751,
                },
                fuzz: 0.29437894,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.2899551,
                y: 0.2,
                z: -2.8386807,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.024613546,
                    g: 0.15567198,
                    b: 0.14457993,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.0388327,
                y: 0.2,
                z: -1.7419686,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.5018364,
                    g: 0.53669333,
                    b: 0.90722907,
                },
                fuzz: 0.39320922,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.8104448,
                y: 0.2,
                z: -0.8797376,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.15021507,
                    g: 0.1366918,
                    b: 0.49570146,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.5914862,
                y: 0.2,
                z: 0.03267499,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.67320794,
                    g: 0.49391356,
                    b: 0.25384057,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.2868788,
                y: 0.2,
                z: 1.1801808,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.24163994,
                    g: 0.25342444,
                    b: 0.019710895,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.1935644,
                y: 0.2,
                z: 2.8542051,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.75943416,
                    g: 0.45169538,
                    b: 0.042062167,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.7261502,
                y: 0.2,
                z: 3.5646765,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.057179715,
                    g: 0.40823394,
                    b: 0.08391046,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.7889645,
                y: 0.2,
                z: 4.6904287,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.07749772,
                    g: 0.020701196,
                    b: 0.018649353,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.8806869,
                y: 0.2,
                z: 5.1847167,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.7063103,
                    g: 0.005365446,
                    b: 0.048037224,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.3549786,
                y: 0.2,
                z: 6.0158343,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.45208606,
                    g: 0.36193272,
                    b: 0.40982166,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.2623944,
                y: 0.2,
                z: 7.113498,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.6487574,
                    g: 0.6055747,
                    b: 0.61254275,
                },
                fuzz: 0.016843647,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.6505128,
                y: 0.2,
                z: 8.679133,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.106117934,
                    g: 0.018359102,
                    b: 0.35632318,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.7879026,
                y: 0.2,
                z: 9.288343,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.107126966,
                    g: 0.27820516,
                    b: 0.10432685,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 1.2269833,
                y: 0.2,
                z: 10.55275,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.30620474,
                    g: 0.04870884,
                    b: 0.5547173,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.4844027,
                y: 0.2,
                z: -10.940319,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.35695487,
                    g: 0.558783,
                    b: 0.11059553,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.1409283,
                y: 0.2,
                z: -9.343781,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7572852,
                    g: 0.5340991,
                    b: 0.9443269,
                },
                fuzz: 0.43248194,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.511962,
                y: 0.2,
                z: -8.327469,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8480794,
                    g: 0.8891828,
                    b: 0.62069654,
                },
                fuzz: 0.27409655,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.0030668,
                y: 0.2,
                z: -7.145421,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.34572816,
                    g: 0.07353741,
                    b: 0.09343737,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.4510086,
                y: 0.2,
                z: -6.100472,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.009605268,
                    g: 0.21759759,
                    b: 0.82430255,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.046321,
                y: 0.2,
                z: -5.8730736,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.15510093,
                    g: 0.029229412,
                    b: 0.32353687,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.1073983,
                y: 0.2,
                z: -4.951543,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.5166373,
                y: 0.2,
                z: -3.89284,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.4521921,
                    g: 0.038848303,
                    b: 0.04550428,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.554457,
                y: 0.2,
                z: -2.2575948,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.35754168,
                    g: 0.0026227548,
                    b: 0.27121294,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.2804916,
                y: 0.2,
                z: -1.7297778,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.20774625,
                    g: 0.015984258,
                    b: 0.21598603,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.4854064,
                y: 0.2,
                z: -0.91941446,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.17202875,
                    g: 0.0781689,
                    b: 0.08514383,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.4401088,
                y: 0.2,
                z: 0.39825654,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.3155925,
                    g: 0.001904672,
                    b: 0.84210557,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.3070998,
                y: 0.2,
                z: 1.6630229,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.48747417,
                    g: 0.3262034,
                    b: 0.557201,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.7578487,
                y: 0.2,
                z: 2.62608,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.465277,
                    g: 0.057594534,
                    b: 0.027149947,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.4400384,
                y: 0.2,
                z: 3.7259605,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.051941715,
                    g: 0.26875466,
                    b: 0.64935696,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.4631236,
                y: 0.2,
                z: 4.315445,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.008870221,
                    g: 0.067287646,
                    b: 0.010995454,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.26736,
                y: 0.2,
                z: 5.7718587,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7810987,
                    g: 0.67480755,
                    b: 0.9689046,
                },
                fuzz: 0.14373285,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.2267873,
                y: 0.2,
                z: 6.879343,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.37229013,
                    g: 0.2492393,
                    b: 0.6623138,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.789863,
                y: 0.2,
                z: 7.262048,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.051689,
                y: 0.2,
                z: 8.670582,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.1157572,
                y: 0.2,
                z: 9.57984,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.14790075,
                    g: 0.14346553,
                    b: 0.34506598,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 2.4984844,
                y: 0.2,
                z: 10.14705,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.4328384,
                y: 0.2,
                z: -10.144121,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.6290505,
                    g: 0.9932383,
                    b: 0.75370604,
                },
                fuzz: 0.48443934,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.8423193,
                y: 0.2,
                z: -9.962149,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.22234048,
                    g: 0.1722152,
                    b: 0.13167816,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.7626483,
                y: 0.2,
                z: -8.50982,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.309602,
                y: 0.2,
                z: -7.5963707,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.28319028,
                    g: 0.12474709,
                    b: 0.24994521,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.8971279,
                y: 0.2,
                z: -6.69233,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.251195,
                y: 0.2,
                z: -5.8124375,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.59665626,
                    g: 0.28324705,
                    b: 0.775525,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.7009764,
                y: 0.2,
                z: -4.910951,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.71478146,
                    g: 0.9852002,
                    b: 0.7910551,
                },
                fuzz: 0.32155538,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.6002154,
                y: 0.2,
                z: -3.8798158,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.7853246,
                y: 0.2,
                z: -2.7508464,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.5143711,
                    g: 0.30282936,
                    b: 0.13034838,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.8001728,
                y: 0.2,
                z: -1.3219914,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.6250438,
                    g: 0.74496245,
                    b: 0.7941865,
                },
                fuzz: 0.3030414,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.5897775,
                y: 0.2,
                z: 1.7876403,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.36215463,
                    g: 0.18705003,
                    b: 0.36833733,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.4129727,
                y: 0.2,
                z: 2.7786336,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.077099405,
                    g: 0.5487255,
                    b: 0.14661156,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.467234,
                y: 0.2,
                z: 3.5895188,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8829717,
                    g: 0.5264983,
                    b: 0.7767895,
                },
                fuzz: 0.13993129,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.0450935,
                y: 0.2,
                z: 4.687666,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.37853712,
                    g: 0.049347665,
                    b: 0.005498898,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.447346,
                y: 0.2,
                z: 5.466675,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.004521548,
                    g: 0.38676775,
                    b: 0.1594487,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.0690756,
                y: 0.2,
                z: 6.586087,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.40374184,
                    g: 0.097563595,
                    b: 0.2335716,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.8091547,
                y: 0.2,
                z: 7.53397,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.17036012,
                    g: 0.81013316,
                    b: 0.32484525,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.484664,
                y: 0.2,
                z: 8.202729,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.8989472,
                y: 0.2,
                z: 9.627041,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.9682069,
                    g: 0.9366555,
                    b: 0.6220974,
                },
                fuzz: 0.037908643,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 3.337222,
                y: 0.2,
                z: 10.742779,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.6930212,
                    g: 0.9156579,
                    b: 0.6579113,
                },
                fuzz: 0.19399777,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.623419,
                y: 0.2,
                z: -10.370935,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0718829,
                    g: 0.27057797,
                    b: 0.0010843946,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.6171904,
                y: 0.2,
                z: -9.454479,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0023797706,
                    g: 0.10325093,
                    b: 0.08742292,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.328205,
                y: 0.2,
                z: -8.624843,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.6280077,
                    g: 0.74624944,
                    b: 0.8972484,
                },
                fuzz: 0.1395339,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.0454154,
                y: 0.2,
                z: -7.216337,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.31236747,
                    g: 0.6473265,
                    b: 0.059900444,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.0155716,
                y: 0.2,
                z: -6.5844035,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.89568,
                y: 0.2,
                z: -5.606928,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.074649714,
                    g: 0.5872891,
                    b: 0.077896796,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.208119,
                y: 0.2,
                z: -4.2451286,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.08636753,
                    g: 0.0035691431,
                    b: 0.38041136,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.7161303,
                y: 0.2,
                z: -3.8688173,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.453801,
                y: 0.2,
                z: -2.525464,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.11352023,
                    g: 0.39433748,
                    b: 0.12504898,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.2058125,
                y: 0.2,
                z: -1.7364147,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.35981932,
                    g: 0.32590115,
                    b: 0.01788433,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.8397055,
                y: 0.2,
                z: 1.034151,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.046019346,
                    g: 0.28463888,
                    b: 0.17705508,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.820706,
                y: 0.2,
                z: 2.1001997,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8385612,
                    g: 0.83618736,
                    b: 0.69481707,
                },
                fuzz: 0.1218577,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.77275,
                y: 0.2,
                z: 3.0800672,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.08066539,
                    g: 0.12712018,
                    b: 0.035951197,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.4039497,
                y: 0.2,
                z: 4.7075443,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.46422568,
                    g: 0.24064296,
                    b: 0.22211345,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.417774,
                y: 0.2,
                z: 5.890032,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7270846,
                    g: 0.7633915,
                    b: 0.6594883,
                },
                fuzz: 0.22412977,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.6015987,
                y: 0.2,
                z: 6.377012,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.5567653,
                    g: 0.90696096,
                    b: 0.88247275,
                },
                fuzz: 0.33778077,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.6280885,
                y: 0.2,
                z: 7.7089033,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.62263596,
                    g: 0.11769373,
                    b: 0.54318076,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.266067,
                y: 0.2,
                z: 8.601793,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.9332981,
                    g: 0.96088445,
                    b: 0.703678,
                },
                fuzz: 0.44085526,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.127451,
                y: 0.2,
                z: 9.4998865,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.465606,
                    g: 0.2963549,
                    b: 0.054365177,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.1515217,
                y: 0.2,
                z: 10.471719,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.717461,
                    g: 0.6685022,
                    b: 0.53052926,
                },
                fuzz: 0.11591709,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.2220187,
                y: 0.2,
                z: -10.287845,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.17469838,
                    g: 0.09783737,
                    b: 0.14304128,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.663535,
                y: 0.2,
                z: -9.883004,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.4528746,
                    g: 0.2532689,
                    b: 0.460472,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.362878,
                y: 0.2,
                z: -8.587675,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.5858308,
                    g: 0.3948258,
                    b: 0.1461599,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.2345333,
                y: 0.2,
                z: -7.260882,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.5814543,
                    g: 0.5320224,
                    b: 0.81668335,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.851723,
                y: 0.2,
                z: -6.2771306,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.03947879,
                    g: 0.017943291,
                    b: 0.3225379,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.306098,
                y: 0.2,
                z: -5.691275,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.08936176,
                    g: 0.29747266,
                    b: 0.17967162,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.44857,
                y: 0.2,
                z: -4.718155,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.25665787,
                    g: 0.37318382,
                    b: 0.4040147,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.5344534,
                y: 0.2,
                z: -3.4931324,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.014529587,
                    g: 0.010187733,
                    b: 0.4975678,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.482888,
                y: 0.2,
                z: -2.9441478,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.4524218,
                    g: 0.082969256,
                    b: 0.14567022,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.4002194,
                y: 0.2,
                z: -1.6813653,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7880137,
                    g: 0.5558696,
                    b: 0.5320251,
                },
                fuzz: 0.4919284,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.5216594,
                y: 0.2,
                z: -0.2460782,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.2782581,
                    g: 0.26362592,
                    b: 0.3200307,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.5726495,
                y: 0.2,
                z: 0.61548036,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.12258564,
                    g: 0.517471,
                    b: 0.43024254,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.6524124,
                y: 0.2,
                z: 1.1330615,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.3194759,
                    g: 0.32606032,
                    b: 0.21535552,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.567963,
                y: 0.2,
                z: 2.0168827,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.97437185,
                    g: 0.7207707,
                    b: 0.6234832,
                },
                fuzz: 0.2425943,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.070529,
                y: 0.2,
                z: 3.6839447,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.36233085,
                    g: 0.015929172,
                    b: 0.39166892,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.789715,
                y: 0.2,
                z: 4.7695093,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.5105873,
                    g: 0.51770866,
                    b: 0.916242,
                },
                fuzz: 0.111121714,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.6066923,
                y: 0.2,
                z: 5.0582175,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.026381074,
                    g: 0.12341052,
                    b: 0.18921517,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.511523,
                y: 0.2,
                z: 6.6891866,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.87088144,
                    g: 0.5766651,
                    b: 0.576588,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.8320684,
                y: 0.2,
                z: 7.4457645,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.8723596,
                    g: 0.69102925,
                    b: 0.17293224,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.5934324,
                y: 0.2,
                z: 8.308536,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.033500254,
                    g: 0.3522183,
                    b: 0.055837967,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.774628,
                y: 0.2,
                z: 9.447256,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.33936563,
                    g: 0.4216266,
                    b: 0.058069337,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 5.594848,
                y: 0.2,
                z: 10.39362,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.31503955,
                    g: 0.24558854,
                    b: 0.02990256,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.3197255,
                y: 0.2,
                z: -10.94289,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.5691353,
                    g: 0.6060685,
                    b: 0.859461,
                },
                fuzz: 0.35636595,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.2868743,
                y: 0.2,
                z: -9.264664,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.2641599,
                    g: 0.55395466,
                    b: 0.08951807,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.3645773,
                y: 0.2,
                z: -8.755916,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.07085744,
                    g: 0.29808316,
                    b: 0.042202443,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.8785768,
                y: 0.2,
                z: -7.7526083,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.005442304,
                    g: 0.5964985,
                    b: 0.23722403,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.418324,
                y: 0.2,
                z: -6.6276817,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.18786213,
                    g: 0.033902127,
                    b: 0.71885645,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.5164127,
                y: 0.2,
                z: -5.814086,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.47079253,
                    g: 0.013869963,
                    b: 0.2557115,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.5365915,
                y: 0.2,
                z: -4.232927,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.025450313,
                    g: 0.0018392107,
                    b: 0.14775345,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.4551926,
                y: 0.2,
                z: -3.207212,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.019995073,
                    g: 0.70861393,
                    b: 0.1823177,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.233895,
                y: 0.2,
                z: -2.2970371,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.6202352,
                    g: 0.602873,
                    b: 0.9188531,
                },
                fuzz: 0.18642476,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.13865,
                y: 0.2,
                z: -1.79813,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.11141607,
                    g: 0.09609463,
                    b: 0.047068853,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.868635,
                y: 0.2,
                z: -0.80822015,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.112008326,
                    g: 0.068243265,
                    b: 0.0031154088,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.0939465,
                y: 0.2,
                z: 0.18220213,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.2525294,
                    g: 0.23979995,
                    b: 0.7814311,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.4029994,
                y: 0.2,
                z: 1.7692465,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.042314865,
                    g: 0.017026918,
                    b: 0.033024598,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.7490478,
                y: 0.2,
                z: 2.4897242,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0035672572,
                    g: 0.15929335,
                    b: 0.32310355,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.517411,
                y: 0.2,
                z: 3.7996547,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.315379,
                y: 0.2,
                z: 4.4019523,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.02382592,
                    g: 0.76534617,
                    b: 0.54421145,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.336945,
                y: 0.2,
                z: 5.407332,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.16429892,
                    g: 0.7735443,
                    b: 0.42601737,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.527889,
                y: 0.2,
                z: 6.6316442,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.015978176,
                    g: 0.0043922192,
                    b: 0.061935764,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.25241,
                y: 0.2,
                z: 7.3175364,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.16718327,
                    g: 0.49665445,
                    b: 0.101304464,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.84133,
                y: 0.2,
                z: 8.774881,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.11626186,
                    g: 0.015732395,
                    b: 0.23013364,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.2231054,
                y: 0.2,
                z: 9.379408,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.43024755,
                    g: 0.03889754,
                    b: 0.2694169,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 6.5761433,
                y: 0.2,
                z: 10.587805,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.65393674,
                    g: 0.52925056,
                    b: 0.1242687,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.8429165,
                y: 0.2,
                z: -10.913674,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.28836122,
                    g: 0.19184065,
                    b: 0.9069059,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.473903,
                y: 0.2,
                z: -9.678713,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.14014137,
                    g: 0.08038579,
                    b: 0.4428566,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.7636065,
                y: 0.2,
                z: -8.814365,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.37495196,
                    g: 0.09657078,
                    b: 0.011326605,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.7152243,
                y: 0.2,
                z: -7.910143,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.66479856,
                    g: 0.61051047,
                    b: 0.76880145,
                },
                fuzz: 0.0024448633,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.129233,
                y: 0.2,
                z: -6.106942,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.010924102,
                    g: 0.9240617,
                    b: 0.0025829826,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.6592712,
                y: 0.2,
                z: -5.5278506,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.093287915,
                    g: 0.3599121,
                    b: 0.57746685,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.791063,
                y: 0.2,
                z: -4.7461734,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.2553635,
                y: 0.2,
                z: -3.5235162,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.636196,
                    g: 0.049676836,
                    b: 0.3410604,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.356872,
                y: 0.2,
                z: -2.7695146,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.63468903,
                    g: 0.27466762,
                    b: 0.06799681,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.489555,
                y: 0.2,
                z: -1.8765752,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.40931332,
                    g: 0.17862001,
                    b: 0.65922546,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.348027,
                y: 0.2,
                z: -0.7073379,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.08343955,
                    g: 0.1484259,
                    b: 0.43030995,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.628261,
                y: 0.2,
                z: 0.79063547,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.29419667,
                    g: 0.12097081,
                    b: 0.6270664,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.0094576,
                y: 0.2,
                z: 1.6878068,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.3067162,
                    g: 0.6577058,
                    b: 0.17842884,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.4640794,
                y: 0.2,
                z: 2.1179414,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.002770583,
                    g: 0.078822,
                    b: 0.025521943,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.0733275,
                y: 0.2,
                z: 3.6304667,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0053130123,
                    g: 0.5163272,
                    b: 0.3131284,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.6258173,
                y: 0.2,
                z: 4.6907806,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.101732716,
                    g: 0.3151333,
                    b: 0.04732625,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.845254,
                y: 0.2,
                z: 5.635155,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.658811,
                y: 0.2,
                z: 6.8145056,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.97425675,
                    g: 0.69846135,
                    b: 0.51029646,
                },
                fuzz: 0.29469565,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.649164,
                y: 0.2,
                z: 7.5403714,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.46271285,
                    g: 0.81843007,
                    b: 0.122892484,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.4935803,
                y: 0.2,
                z: 8.696676,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.20404842,
                    g: 0.111523144,
                    b: 0.15857226,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.737658,
                y: 0.2,
                z: 9.165884,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.057088487,
                    g: 0.14976913,
                    b: 0.34281716,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 7.228851,
                y: 0.2,
                z: 10.814601,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.17365779,
                    g: 0.03890009,
                    b: 0.19345829,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.762279,
                y: 0.2,
                z: -10.17554,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.10351902,
                    g: 0.031768937,
                    b: 0.13986684,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.288315,
                y: 0.2,
                z: -9.240248,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.07030642,
                    g: 0.19588391,
                    b: 0.07958294,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.21761,
                y: 0.2,
                z: -8.392436,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.5398094,
                    g: 0.054774735,
                    b: 0.31124043,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.879608,
                y: 0.2,
                z: -7.324954,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.21535079,
                    g: 0.29539168,
                    b: 0.049108382,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.059459,
                y: 0.2,
                z: -6.802519,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.15205,
                y: 0.2,
                z: -5.2844715,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.21819305,
                    g: 0.29545444,
                    b: 0.05223575,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.3924,
                y: 0.2,
                z: -4.759714,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0059552556,
                    g: 0.40531462,
                    b: 0.06728474,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.174431,
                y: 0.2,
                z: -3.730907,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.196606,
                    g: 0.056114506,
                    b: 0.113255024,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.760475,
                y: 0.2,
                z: -2.5784857,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.4042937,
                    g: 0.006141634,
                    b: 0.3196978,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.668914,
                y: 0.2,
                z: -1.6303015,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.68915087,
                    g: 0.98544264,
                    b: 0.5532948,
                },
                fuzz: 0.068115175,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.729033,
                y: 0.2,
                z: -0.81163156,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8104475,
                    g: 0.53148687,
                    b: 0.9372765,
                },
                fuzz: 0.40680534,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.191136,
                y: 0.2,
                z: 0.28738025,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.0061288965,
                    g: 0.065900706,
                    b: 0.043071225,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.277855,
                y: 0.2,
                z: 1.4101654,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.1103649,
                    g: 0.18339488,
                    b: 0.67681587,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.249439,
                y: 0.2,
                z: 2.4727206,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.105553925,
                    g: 0.94544536,
                    b: 0.055251013,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.736203,
                y: 0.2,
                z: 3.4246957,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.18972631,
                    g: 0.06722246,
                    b: 0.6588793,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.17642,
                y: 0.2,
                z: 4.2530546,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.8219808,
                    g: 0.7683482,
                    b: 0.9223299,
                },
                fuzz: 0.38384488,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.858156,
                y: 0.2,
                z: 5.0887656,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.2945599,
                    g: 0.29801428,
                    b: 0.36687174,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.318577,
                y: 0.2,
                z: 6.893401,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.028291436,
                    g: 0.05330672,
                    b: 0.5449489,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.811484,
                y: 0.2,
                z: 7.5505323,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.6413803,
                    g: 0.0987533,
                    b: 0.08964722,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.139104,
                y: 0.2,
                z: 8.431983,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.9059341,
                    g: 0.8421178,
                    b: 0.7287334,
                },
                fuzz: 0.4183712,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.18722,
                y: 0.2,
                z: 9.644417,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.6420333,
                    g: 0.263933,
                    b: 0.22766994,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 8.489182,
                y: 0.2,
                z: 10.803768,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.39688975,
                    g: 0.47315165,
                    b: 0.39477083,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.53539,
                y: 0.2,
                z: -10.235397,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.03694926,
                    g: 0.107048415,
                    b: 0.6591502,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.312154,
                y: 0.2,
                z: -9.319298,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.028125484,
                    g: 0.39874825,
                    b: 0.33579987,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.132294,
                y: 0.2,
                z: -8.51421,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.27485332,
                    g: 0.36547005,
                    b: 0.3616934,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.740648,
                y: 0.2,
                z: -7.802813,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.45897284,
                    g: 0.089921415,
                    b: 0.64100325,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.146712,
                y: 0.2,
                z: -6.1379595,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.085548,
                y: 0.2,
                z: -5.9169292,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.5319532,
                    g: 0.46344313,
                    b: 0.1223182,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.239831,
                y: 0.2,
                z: -4.881304,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.570078,
                    g: 0.7511488,
                    b: 0.16688919,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.083654,
                y: 0.2,
                z: -3.6190174,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7883718,
                    g: 0.9695718,
                    b: 0.70330215,
                },
                fuzz: 0.37426135,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.818649,
                y: 0.2,
                z: -2.7092981,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.10660683,
                    g: 0.42446473,
                    b: 0.14185745,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.156014,
                y: 0.2,
                z: -1.8576334,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7703516,
                    g: 0.6564919,
                    b: 0.9498552,
                },
                fuzz: 0.17795035,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.543936,
                y: 0.2,
                z: -0.72583044,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.7757326,
                    g: 0.001926377,
                    b: 0.23695561,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.352524,
                y: 0.2,
                z: 0.3511679,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.07075167,
                    g: 0.19566908,
                    b: 0.2293058,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.050702,
                y: 0.2,
                z: 1.730597,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.848812,
                    g: 0.752228,
                    b: 0.84003323,
                },
                fuzz: 0.24866018,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.710659,
                y: 0.2,
                z: 2.343967,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.87140405,
                    g: 0.59323573,
                    b: 0.8088664,
                },
                fuzz: 0.17752287,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.134368,
                y: 0.2,
                z: 3.1833365,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.207515,
                y: 0.2,
                z: 4.518894,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.22399832,
                    g: 0.38559395,
                    b: 0.029809227,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.117067,
                y: 0.2,
                z: 5.5347757,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.07280143,
                    g: 0.21444778,
                    b: 0.57192725,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.06171,
                y: 0.2,
                z: 6.216476,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.95403326,
                    g: 0.5599811,
                    b: 0.57203233,
                },
                fuzz: 0.22457752,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.3447,
                y: 0.2,
                z: 7.4746356,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.068459466,
                    g: 0.10575422,
                    b: 0.08637003,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.167372,
                y: 0.2,
                z: 8.755372,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.17736572,
                    g: 0.107227765,
                    b: 0.033044375,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.35577,
                y: 0.2,
                z: 9.784779,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.39615297,
                    g: 0.2712522,
                    b: 0.00055377383,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 9.233131,
                y: 0.2,
                z: 10.575851,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.604509,
                    g: 0.18285704,
                    b: 0.5301299,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.716796,
                y: 0.2,
                z: -10.709762,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.4987421,
                    g: 0.4380448,
                    b: 0.2484264,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.822479,
                y: 0.2,
                z: -9.71849,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.757388,
                y: 0.2,
                z: -8.389286,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.17038883,
                    g: 0.04248063,
                    b: 0.7514598,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.623192,
                y: 0.2,
                z: -7.6498528,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.837343,
                y: 0.2,
                z: -6.93344,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.14602454,
                    g: 0.8410272,
                    b: 0.36624542,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.285543,
                y: 0.2,
                z: -5.9839377,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.3341868,
                    g: 0.043173887,
                    b: 0.27181116,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.809701,
                y: 0.2,
                z: -4.6129165,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.44439963,
                    g: 0.2779921,
                    b: 0.3470523,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.023335,
                y: 0.2,
                z: -3.7922423,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.10744256,
                    g: 0.54215497,
                    b: 0.050190818,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.743953,
                y: 0.2,
                z: -2.8604395,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.78157234,
                    g: 0.11763592,
                    b: 0.06884451,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.233315,
                y: 0.2,
                z: -1.8360854,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.17736356,
                    g: 0.23212266,
                    b: 0.53503793,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.879175,
                y: 0.2,
                z: -0.48520958,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.043391053,
                    g: 0.09567228,
                    b: 0.42155313,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.3011675,
                y: 0.2,
                z: 0.6003546,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.11383814,
                    g: 0.008815439,
                    b: 0.052374475,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.56965,
                y: 0.2,
                z: 1.0805551,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.3039443,
                    g: 0.22552958,
                    b: 0.24832726,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.819919,
                y: 0.2,
                z: 2.6368032,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.61975884,
                    g: 0.3798178,
                    b: 0.3132943,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.66252,
                y: 0.2,
                z: 3.5289733,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.055700306,
                    g: 0.14331913,
                    b: 0.7961446,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.6237955,
                y: 0.2,
                z: 4.719553,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.65388,
                    g: 0.8642443,
                    b: 0.83584243,
                },
                fuzz: 0.25893837,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.161772,
                y: 0.2,
                z: 5.2144537,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.1262718,
                    g: 0.06758793,
                    b: 0.19749829,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.792396,
                y: 0.2,
                z: 6.3364277,
            },
            radius: 0.2,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.365405,
                y: 0.2,
                z: 7.843898,
            },
            radius: 0.2,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.9539437,
                    g: 0.5038402,
                    b: 0.6098592,
                },
                fuzz: 0.21936089,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.626994,
                y: 0.2,
                z: 8.735191,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.1409692,
                    g: 0.007180408,
                    b: 0.029327735,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.115327,
                y: 0.2,
                z: 9.691763,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.47659302,
                    g: 0.51478845,
                    b: 0.08051816,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 10.087803,
                y: 0.2,
                z: 10.549803,
            },
            radius: 0.2,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.023383642,
                    g: 0.03495181,
                    b: 0.5371339,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
            material: Arc::new(DialectricMaterial {
                ir: 1.5,
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: -4.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
            material: Arc::new(LambertianMaterial {
                albedo: Colour {
                    r: 0.4,
                    g: 0.2,
                    b: 0.1,
                },
            }),
        },
    ),
    HittableObject::Sphere(
        Sphere {
            centre: Vec3 {
                x: 4.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
            material: Arc::new(MetalMaterial {
                albedo: Colour {
                    r: 0.7,
                    g: 0.6,
                    b: 0.5,
                },
                fuzz: 0.0,
            }),
        },
    ),
    ];
    world
}